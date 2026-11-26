//! Mac 管理服务：读取、控制和期限调度分开，原生阻塞不占用 UI。
use super::{
    Action, Capability, Command, ConnectionState, Event, OperationResult, OperationStatus, Request,
    ScanStatus, Snapshot,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::time::{Duration, Instant};

pub const OPERATION_TIMEOUT: Duration = Duration::from_secs(20);

pub struct Service {
    pub rx: mpsc::Receiver<Event>,
    commands: mpsc::SyncSender<Command>,
    stop: Arc<AtomicBool>,
}
impl Service {
    pub fn send(&self, command: Command) -> Result<(), String> {
        self.commands
            .try_send(command)
            .map_err(|error| match error {
                mpsc::TrySendError::Full(_) => "蓝牙服务忙，请稍后重试".into(),
                mpsc::TrySendError::Disconnected(_) => "蓝牙服务已停止，请刷新后重试".into(),
            })
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(target_os = "macos")]
pub fn spawn() -> Service {
    let mut adapter = super::macos::Adapter::new();
    spawn_with(
        move |refresh| adapter.query(refresh),
        super::macos::control,
        OPERATION_TIMEOUT,
    )
}

#[cfg(target_os = "macos")]
pub fn request_access() -> Result<(), String> {
    super::macos::request_access()
}

/// 原生调用结束前保持 occupied；UI 终态不能释放底层工作槽。
struct Pending {
    request: Request,
    at: Instant,
}

fn operation(request: Request, status: OperationStatus) -> Event {
    Event::Operation(OperationResult { request, status })
}

fn target_reached(snapshot: &Snapshot, request: &Request) -> bool {
    matches!(snapshot.status, ScanStatus::Ready | ScanStatus::Partial(_))
        && snapshot.devices.iter().any(|device| {
            device.id == request.device_id
                && device.connection
                    == match request.action {
                        Action::Connect => ConnectionState::Connected,
                        Action::Disconnect => ConnectionState::Disconnected,
                    }
        })
}

fn validate(snapshot: &Snapshot, request: &Request) -> Result<(), String> {
    if !matches!(snapshot.status, ScanStatus::Ready | ScanStatus::Partial(_)) {
        return Err("蓝牙状态尚不可用，请刷新或打开系统设置".into());
    }
    let device = snapshot
        .devices
        .iter()
        .find(|device| device.id == request.device_id)
        .ok_or_else(|| "设备已不在列表，请刷新后重试".to_owned())?;
    if device.connection == ConnectionState::Unknown {
        return Err("设备状态未知，请刷新后重试".into());
    }
    match match request.action {
        Action::Connect => &device.connect,
        Action::Disconnect => &device.disconnect,
    } {
        Capability::Available => Ok(()),
        Capability::Unavailable(reason) => Err(reason.clone()),
        Capability::Unverified => Err("此设备尚不支持直接操作，请打开 Mac 蓝牙设置".into()),
    }
}

fn spawn_with(
    mut query: impl FnMut(bool) -> Snapshot + Send + 'static,
    control: impl Fn(Request) -> Result<(), String> + Send + 'static,
    timeout: Duration,
) -> Service {
    let (events, rx) = mpsc::sync_channel(32);
    let (commands, received) = mpsc::sync_channel(8);
    let stop = Arc::new(AtomicBool::new(false));
    let query_stop = stop.clone();
    let (refresh, refresh_rx) = mpsc::sync_channel(1);
    let (observed, observations) = mpsc::sync_channel(8);
    let query_spawn = std::thread::Builder::new()
        .name("daozai-bt-observe".into())
        .spawn(move || {
            let mut force = true;
            while !query_stop.load(Ordering::Relaxed) {
                let snapshot = query(force);
                if query_stop.load(Ordering::Relaxed) || observed.send(snapshot).is_err() {
                    break;
                }
                match refresh_rx.recv_timeout(Duration::from_secs(2)) {
                    Ok(()) => force = true,
                    Err(mpsc::RecvTimeoutError::Timeout) => force = false,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    let control_stop = stop.clone();
    let (native, native_rx) = mpsc::sync_channel::<Request>(1);
    let (finished, results) = mpsc::sync_channel(1);
    let control_spawn = std::thread::Builder::new()
        .name("daozai-bt-control".into())
        .spawn(move || {
            while let Ok(request) = native_rx.recv() {
                if control_stop.load(Ordering::Relaxed) {
                    break;
                }
                let result = control(request.clone());
                if control_stop.load(Ordering::Relaxed) || finished.send((request, result)).is_err()
                {
                    break;
                }
            }
        });
    let thread_stop = stop.clone();
    let started = query_spawn.is_ok() && control_spawn.is_ok();
    let event_copy = events.clone();
    let scheduled = std::thread::Builder::new()
        .name("daozai-bt-schedule".into())
        .spawn(move || {
            if !started {
                let _ = events.send(Event::Snapshot(Snapshot {
                    status: ScanStatus::Unavailable("无法启动蓝牙后台服务".into()),
                    ..Default::default()
                }));
                return;
            }
            let mut latest = Snapshot::default();
            let mut pending: Option<Pending> = None;
            let mut occupied = false;
            let mut version = 0;
            while !thread_stop.load(Ordering::Relaxed) {
                while let Ok((request, result)) = results.try_recv() {
                    occupied = false;
                    if pending.as_ref().is_some_and(|p| p.request.id == request.id) {
                        if let Err(reason) = result {
                            pending = None;
                            if events
                                .send(operation(request, OperationStatus::Failed(reason)))
                                .is_err()
                            {
                                return;
                            }
                        } else {
                            let _ = refresh.try_send(());
                        }
                    }
                }
                while let Ok(mut snapshot) = observations.try_recv() {
                    version += 1;
                    snapshot.version = version;
                    if let Some(operation) = &pending {
                        if target_reached(&snapshot, &operation.request) {
                            let request = operation.request.clone();
                            pending = None;
                            if events
                                .send(self::operation(request, OperationStatus::Succeeded))
                                .is_err()
                            {
                                return;
                            }
                        } else if snapshot.complete()
                            && !snapshot
                                .devices
                                .iter()
                                .any(|d| d.id == operation.request.device_id)
                        {
                            let request = operation.request.clone();
                            pending = None;
                            if events
                                .send(self::operation(
                                    request,
                                    OperationStatus::Failed("设备已移除".into()),
                                ))
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                    latest = snapshot.clone();
                    if events.send(Event::Snapshot(snapshot)).is_err() {
                        return;
                    }
                }
                if pending.as_ref().is_some_and(|p| p.at.elapsed() >= timeout) {
                    let request = pending.take().unwrap().request;
                    if events
                        .send(operation(request, OperationStatus::TimedOut))
                        .is_err()
                    {
                        return;
                    }
                }
                match received.recv_timeout(Duration::from_millis(50)) {
                    Ok(Command::Refresh) => {
                        let _ = refresh.try_send(());
                    }
                    Ok(Command::Control(request)) => {
                        let error = if occupied || pending.is_some() {
                            Some("上一次蓝牙操作仍在结束，请稍后重试".to_owned())
                        } else {
                            validate(&latest, &request).err()
                        };
                        if let Some(error) = error {
                            if events
                                .send(operation(request, OperationStatus::Failed(error)))
                                .is_err()
                            {
                                return;
                            }
                        } else if target_reached(&latest, &request) {
                            if events
                                .send(operation(request, OperationStatus::Succeeded))
                                .is_err()
                            {
                                return;
                            }
                        } else if native.try_send(request.clone()).is_ok() {
                            occupied = true;
                            pending = Some(Pending {
                                request: request.clone(),
                                at: Instant::now(),
                            });
                            if events
                                .send(operation(request, OperationStatus::Accepted))
                                .is_err()
                            {
                                return;
                            }
                            let _ = refresh.try_send(());
                        } else if events
                            .send(operation(
                                request,
                                OperationStatus::Failed("蓝牙控制线程不可用".into()),
                            ))
                            .is_err()
                        {
                            return;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
            }
        });
    if scheduled.is_err() {
        stop.store(true, Ordering::Relaxed);
        let _ = event_copy.try_send(Event::Snapshot(Snapshot {
            status: ScanStatus::Unavailable("无法启动蓝牙调度服务".into()),
            ..Default::default()
        }));
    }
    Service { rx, commands, stop }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bt::{Device, DeviceKind};
    fn snapshot() -> Snapshot {
        Snapshot::ready(
            1,
            vec![Device {
                id: "A".into(),
                kind: DeviceKind::Keyboard,
                connection: ConnectionState::Connected,
                disconnect: Capability::Available,
                ..Default::default()
            }],
        )
    }
    #[test]
    fn blocked_native_call_times_out_without_spawning_duplicate_calls() {
        let (release, wait) = mpsc::channel();
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count = calls.clone();
        let gate = std::sync::Mutex::new(wait);
        let service = spawn_with(
            |_| snapshot(),
            move |_| {
                count.fetch_add(1, Ordering::Relaxed);
                let _ = gate.lock().unwrap().recv_timeout(Duration::from_secs(2));
                Ok(())
            },
            Duration::from_millis(100),
        );
        assert!(matches!(
            service.rx.recv_timeout(Duration::from_secs(2)).unwrap(),
            Event::Snapshot(_)
        ));
        let req = Request {
            id: 1,
            device_id: "A".into(),
            action: Action::Disconnect,
        };
        service.send(Command::Control(req.clone())).unwrap();
        let until = Instant::now() + Duration::from_secs(2);
        let mut timed_out = false;
        while Instant::now() < until {
            if let Ok(Event::Operation(result)) =
                service.rx.recv_timeout(Duration::from_millis(200))
            {
                if result.status == OperationStatus::TimedOut {
                    timed_out = true;
                    break;
                }
            }
        }
        assert!(
            timed_out,
            "UI deadline must not wait for blocked native control"
        );
        service
            .send(Command::Control(Request { id: 2, ..req }))
            .unwrap();
        loop {
            if let Event::Operation(result) =
                service.rx.recv_timeout(Duration::from_secs(2)).unwrap()
            {
                if result.request.id == 2 {
                    assert!(matches!(result.status, OperationStatus::Failed(_)));
                    break;
                }
            }
        }
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        release.send(()).unwrap();
    }
    #[test]
    fn permissions_and_unknown_states_cannot_control_devices() {
        let request = Request {
            id: 1,
            device_id: "A".into(),
            action: Action::Disconnect,
        };
        let mut value = snapshot();
        assert!(validate(&value, &request).is_ok());
        value.status = ScanStatus::PermissionRequired("denied".into());
        assert!(validate(&value, &request).is_err());
        value.status = ScanStatus::Ready;
        value.devices[0].connection = ConnectionState::Unknown;
        assert!(validate(&value, &request).is_err());
    }
}
