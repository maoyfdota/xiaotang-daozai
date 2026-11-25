//! Native Mac adapter. Only values with authoritative identities leave this module.
// objc 0.2 macros inspect the obsolete cargo-clippy cfg in the calling crate.
#![allow(unexpected_cfgs)]
use super::{
    device_kind, normalize_address, Action, BatteryStatus, Capability, ConnectionState, Device,
    Endpoint, Request, ScanStatus, Snapshot,
};
use objc::runtime::{Object, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::Read,
    process::{Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

#[link(name = "IOBluetooth", kind = "framework")]
extern "C" {}
#[link(name = "CoreBluetooth", kind = "framework")]
extern "C" {}
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

struct Pool(*mut Object);
impl Pool {
    fn new() -> Self {
        Self(unsafe { msg_send![class!(NSAutoreleasePool), new] })
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, drain];
        }
    }
}
struct Manager(*mut Object);
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}
thread_local! { static ACCESS: RefCell<Option<Manager>> = const {RefCell::new(None)}; }
// A dropped service can leave a blocking native call alive. A later service
// generation must reject control instead of entering a second native call.
static CONTROL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn authorization() -> i64 {
    unsafe { msg_send![class!(CBManager), authorization] }
}
fn permission() -> Result<(), ScanStatus> {
    match authorization() {
        3 => Ok(()),
        0 => Err(ScanStatus::PermissionRequired(
            "需要允许岛仔访问蓝牙，请点击「允许访问蓝牙」".into(),
        )),
        1 => Err(ScanStatus::PermissionRequired(
            "系统限制了蓝牙访问，请检查隐私设置".into(),
        )),
        _ => Err(ScanStatus::PermissionRequired(
            "蓝牙权限已关闭，请在系统设置中允许岛仔访问蓝牙".into(),
        )),
    }
}

/// Called by an explicit UI action from the packaged application.
pub(super) fn request_access() -> Result<(), String> {
    let _pool = Pool::new();
    match authorization() {
        3 => return Ok(()),
        1 | 2 => return Err("请在系统设置 → 隐私与安全性 → 蓝牙中允许岛仔访问".into()),
        _ => {}
    }
    unsafe {
        let bundle: *mut Object = msg_send![class!(NSBundle), mainBundle];
        let key = nsstring("NSBluetoothAlwaysUsageDescription");
        let value: *mut Object = msg_send![bundle,objectForInfoDictionaryKey:key];
        if value.is_null() {
            return Err("请从已安装的 Daozai.app 打开蓝牙功能".into());
        }
    }
    ACCESS.with(|slot| {
        let mut slot=slot.borrow_mut();
        if slot.is_none() {
            let manager:*mut Object=unsafe{msg_send![class!(CBCentralManager),alloc]};
            let manager:*mut Object=unsafe{msg_send![manager,initWithDelegate:std::ptr::null_mut::<Object>() queue:std::ptr::null_mut::<Object>()]};
            if manager.is_null(){return Err("无法请求蓝牙权限，请打开系统设置".into());}
            *slot=Some(Manager(manager));
        }
        Ok(())
    })
}

unsafe fn string(value: *mut Object) -> String {
    if value.is_null() {
        return String::new();
    }
    let pointer: *const std::ffi::c_char = msg_send![value, UTF8String];
    if pointer.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(pointer)
            .to_string_lossy()
            .into_owned()
    }
}
unsafe fn nsstring(value: &str) -> *mut Object {
    let obj: *mut Object = msg_send![class!(NSString), alloc];
    let obj: *mut Object =
        msg_send![obj,initWithBytes:value.as_ptr() length:value.len() encoding:4usize];
    msg_send![obj, autorelease]
}

fn native_devices() -> Result<Vec<Device>, ScanStatus> {
    permission()?;
    let _pool = Pool::new();
    unsafe {
        let controller: *mut Object =
            msg_send![class!(IOBluetoothHostController), defaultController];
        if controller.is_null() {
            return Err(ScanStatus::Unavailable("未检测到蓝牙适配器".into()));
        }
        let power: i32 = msg_send![controller, powerState];
        if power != 1 {
            return Err(ScanStatus::Unavailable(
                "Mac 蓝牙已关闭，请在系统设置中打开".into(),
            ));
        }
        let paired: *mut Object = msg_send![class!(IOBluetoothDevice), pairedDevices];
        if paired.is_null() {
            return Ok(Vec::new());
        }
        let count: usize = msg_send![paired, count];
        let mut devices = Vec::new();
        for index in 0..count.min(4096) {
            let object: *mut Object = msg_send![paired,objectAtIndex:index];
            let paired: BOOL = msg_send![object, isPaired];
            if paired != YES {
                continue;
            }
            let raw = string(msg_send![object, addressString]);
            let Some(address) = normalize_address(&raw) else {
                continue;
            };
            let name = string(msg_send![object, name]);
            let class: u32 = msg_send![object, classOfDevice];
            let connected: BOOL = msg_send![object, isConnected];
            // Only classic devices have a documented baseband control path here.
            let classic = class != 0;
            let capability = if classic {
                Capability::Available
            } else {
                Capability::Unavailable("此低功耗设备请使用 Mac 蓝牙设置连接或断开".into())
            };
            devices.push(Device {
                id: address.clone(),
                kind: device_kind(&name, true, Some(class)),
                name: if name.trim().is_empty() {
                    "未命名蓝牙设备".into()
                } else {
                    name
                },
                connection: if connected == YES {
                    ConnectionState::Connected
                } else {
                    ConnectionState::Disconnected
                },
                connect: capability.clone(),
                disconnect: capability,
                endpoints: vec![Endpoint {
                    system_id: raw,
                    address: Some(address),
                    container: None,
                }],
                ..Default::default()
            });
        }
        Ok(devices)
    }
}

pub(super) fn control(request: Request) -> Result<(), String> {
    let _slot = CONTROL
        .try_lock()
        .map_err(|_| "上一次蓝牙操作仍在结束，请稍后重试".to_owned())?;
    permission().map_err(|_| "蓝牙访问权限不可用".to_owned())?;
    let _pool = Pool::new();
    let Some(address) = normalize_address(&request.device_id) else {
        return Err("设备标识无效".into());
    };
    let address = address
        .as_bytes()
        .chunks(2)
        .map(|p| std::str::from_utf8(p).unwrap())
        .collect::<Vec<_>>()
        .join(":");
    unsafe {
        let object: *mut Object =
            msg_send![class!(IOBluetoothDevice),deviceWithAddressString:nsstring(&address)];
        if object.is_null() {
            return Err("设备已不可用".into());
        }
        let paired: BOOL = msg_send![object, isPaired];
        if paired != YES {
            return Err("设备已取消配对，请刷新列表".into());
        }
        let class: u32 = msg_send![object, classOfDevice];
        if class == 0 {
            return Err("此设备请通过 Mac 蓝牙设置操作".into());
        }
        let connected: BOOL = msg_send![object, isConnected];
        if (connected == YES) == (request.action == Action::Connect) {
            return Ok(());
        }
        let result: i32 = match request.action {
            Action::Connect => {
                msg_send![object,openConnection:std::ptr::null_mut::<Object>() withPageTimeout:16000u16 authenticationRequired:YES]
            }
            Action::Disconnect => msg_send![object, closeConnection],
        };
        if result == 0 {
            Ok(())
        } else {
            Err(format!(
                "系统未完成蓝牙操作（{:#x}），可在 Mac 蓝牙设置中重试",
                result as u32
            ))
        }
    }
}

#[derive(Default)]
struct Report {
    levels: BTreeMap<String, u8>,
}
fn parse_report(bytes: &[u8]) -> Result<Report, String> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| "系统蓝牙报告格式无效".to_owned())?;
    let reports = value
        .get("SPBluetoothDataType")
        .and_then(|v| v.as_array())
        .ok_or("系统蓝牙报告缺少数据")?;
    let mut result = Report::default();
    for root in reports {
        if let Some(records) = root.get("device_connected").and_then(|v| v.as_array()) {
            for record in records {
                let Some(names) = record.as_object() else {
                    continue;
                };
                for fields in names.values() {
                    let Some(id) = fields
                        .get("device_address")
                        .and_then(|v| v.as_str())
                        .and_then(normalize_address)
                    else {
                        continue;
                    };
                    let level = fields
                        .get("device_batteryLevelMain")
                        .or_else(|| fields.get("device_batteryLevel"));
                    if let Some(level) = level.and_then(battery_value) {
                        result.levels.insert(id, level);
                    }
                }
            }
        }
    }
    Ok(result)
}
fn battery_value(value: &serde_json::Value) -> Option<u8> {
    let number = value.as_u64().or_else(|| {
        value
            .as_str()?
            .trim()
            .trim_end_matches('%')
            .parse::<u64>()
            .ok()
    })?;
    (number <= 100).then_some(number as u8)
}

fn read_report() -> Result<Report, String> {
    const LIMIT: u64 = 2 * 1024 * 1024;
    let mut child = Command::new("/usr/sbin/system_profiler")
        .args(["SPBluetoothDataType", "-json", "-detailLevel", "basic"])
        .env("LC_ALL", "C")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "无法读取系统蓝牙电量".to_owned())?;
    let stdout = child.stdout.take().ok_or("无法读取系统蓝牙报告")?;
    let reader = std::thread::spawn(move || {
        let mut data = Vec::new();
        let result = stdout.take(LIMIT + 1).read_to_end(&mut data);
        result.map(|_| data)
    });
    let until = Instant::now() + Duration::from_secs(15);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) if Instant::now() < until => std::thread::sleep(Duration::from_millis(50)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                break Err("系统蓝牙电量读取超时".to_owned());
            }
        }
    };
    let data = reader
        .join()
        .map_err(|_| "系统蓝牙报告读取失败".to_owned())?
        .map_err(|_| "系统蓝牙报告读取失败".to_owned())?;
    if !status?.success() || data.len() as u64 > LIMIT {
        return Err("系统蓝牙报告暂不可用".into());
    }
    parse_report(&data)
}

pub(super) struct Adapter {
    previous: Vec<Device>,
    request: mpsc::SyncSender<Instant>,
    results: mpsc::Receiver<(Instant, Result<Report, String>)>,
    report: Option<(Instant, Result<Report, String>)>,
    connected_at: BTreeMap<String, Instant>,
    requested: Option<Instant>,
    reading: bool,
    sequence: u64,
}
impl Adapter {
    pub(super) fn new() -> Self {
        let (request, wait) = mpsc::sync_channel(1);
        let (send, results) = mpsc::sync_channel(1);
        let _ = std::thread::Builder::new()
            .name("daozai-bt-battery".into())
            .spawn(move || {
                while let Ok(started) = wait.recv() {
                    // Do not bypass a subsequently revoked Bluetooth permission.
                    let report = if permission().is_ok() {
                        read_report()
                    } else {
                        Err("蓝牙权限不可用".into())
                    };
                    if send.send((started, report)).is_err() {
                        break;
                    }
                }
            });
        Self {
            previous: Vec::new(),
            request,
            results,
            report: None,
            connected_at: BTreeMap::new(),
            requested: None,
            reading: false,
            sequence: 0,
        }
    }
    pub(super) fn query(&mut self) -> Snapshot {
        let mut devices = match native_devices() {
            Ok(devices) => devices,
            Err(status) => {
                for device in &mut self.previous {
                    device.connection = ConnectionState::Unknown;
                    device.battery_status = BatteryStatus::Failed;
                }
                self.report = None;
                self.connected_at.clear();
                return Snapshot {
                    devices: self.previous.clone(),
                    status,
                    ..Default::default()
                };
            }
        };
        for device in &devices {
            if device.connection == ConnectionState::Connected
                && !self
                    .previous
                    .iter()
                    .any(|old| old.id == device.id && old.connection == ConnectionState::Connected)
            {
                self.connected_at.insert(device.id.clone(), Instant::now());
            }
        }
        while let Ok(result) = self.results.try_recv() {
            self.sequence += 1;
            self.report = Some(result);
            self.reading = false;
        }
        let changed = devices
            .iter()
            .filter(|d| d.connection == ConnectionState::Connected)
            .map(|d| &d.id)
            .ne(self
                .previous
                .iter()
                .filter(|d| d.connection == ConnectionState::Connected)
                .map(|d| &d.id));
        if !self.reading
            && (changed
                || self
                    .requested
                    .is_none_or(|t| t.elapsed() >= Duration::from_secs(60)))
            && self.request.try_send(Instant::now()).is_ok()
        {
            self.reading = true;
            self.requested = Some(Instant::now());
        }
        for device in &mut devices {
            if device.connection != ConnectionState::Connected {
                continue;
            }
            if !self.report.as_ref().is_some_and(|(started, _)| {
                self.connected_at
                    .get(&device.id)
                    .is_none_or(|at| started >= at)
            }) {
                // A new connection must wait for a post-connection observation.
                continue;
            }
            device.battery_observation = self.sequence;
            match &self.report {
                Some((_, Ok(report))) => {
                    device.battery = report.levels.get(&device.id).copied();
                    device.battery_status = if device.battery.is_some() {
                        BatteryStatus::Valid
                    } else {
                        BatteryStatus::Unknown
                    };
                }
                Some((_, Err(_))) => device.battery_status = BatteryStatus::Failed,
                None => {}
            }
        }
        let status = match &self.report {
            Some((_, Err(reason))) => ScanStatus::Partial(format!("设备状态可用；{reason}")),
            // pairedDevices is authoritative for this native source. Battery errors
            // remain partial, and denied/unavailable sources never imply removal.
            _ => ScanStatus::Ready,
        };
        self.previous = devices.clone();
        Snapshot {
            devices,
            status,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn occupied_control_slot_rejects_a_later_service_generation() {
        let _previous = CONTROL.lock().unwrap();
        let error = control(Request {
            id: 1,
            device_id: "AABBCCDDEE01".into(),
            action: Action::Connect,
        }).unwrap_err();
        assert!(error.contains("上一次蓝牙操作"));
    }

    #[test]
    fn report_only_uses_connected_identity_and_valid_aggregate_battery() {
        let report = serde_json::json!({"SPBluetoothDataType":[{"device_connected":[
            {"same":{"device_address":"AA:BB:CC:DD:EE:01","device_batteryLevelMain":"0%"}},
            {"same":{"device_address":"AA:BB:CC:DD:EE:02","device_batteryLevelMain":100}},
            {"bad":{"device_address":"AA:BB:CC:DD:EE:03","device_batteryLevelMain":"101%"}},
            {"name-only":{"device_batteryLevelMain":"19%"}}
        ],"device_not_connected":[{"offline":{"device_address":"AA:BB:CC:DD:EE:04","device_batteryLevelMain":"20%"}}]}]});
        let parsed = parse_report(&serde_json::to_vec(&report).unwrap()).unwrap();
        assert_eq!(parsed.levels.len(), 2);
        assert_eq!(parsed.levels["AABBCCDDEE01"], 0);
        assert_eq!(parsed.levels["AABBCCDDEE02"], 100);
        assert!(parse_report(b"{}").is_err());
        assert!(parse_report(b"not json").is_err());
    }
}
