//! 只消费新电量观测的设备级提醒周期。排队状态由实际通知队列维护。
use super::{ConnectionState, Device, ScanStatus, Snapshot};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const THRESHOLD: u8 = 20;

#[derive(Default)]
struct Observation {
    sequence: u64,
    cycle: Option<u64>,
    eligible: bool,
}

pub(crate) struct LowBattery {
    observations: BTreeMap<String, Observation>,
    shown: BTreeSet<String>,
    next_cycle: u64,
    enabled: bool,
    dirty: bool,
}

impl LowBattery {
    pub(crate) fn new(shown: BTreeSet<String>, enabled: bool) -> Self {
        Self {
            observations: BTreeMap::new(),
            shown,
            next_cycle: 0,
            enabled,
            dirty: false,
        }
    }

    pub(crate) fn shown(&self) -> &BTreeSet<String> {
        &self.shown
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    pub(crate) fn reset_service(&mut self) {
        self.observations.clear();
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool, current: &Snapshot) {
        if self.enabled == enabled {
            return;
        }
        self.enabled = enabled;
        for observation in self.observations.values_mut() {
            observation.eligible = false;
        }
        // 开启时建立观测基线，已有缓存不能充当开启后的首次成功读取。
        for device in &current.devices {
            let observation = self.observations.entry(device.id.clone()).or_default();
            observation.sequence = observation.sequence.max(device.battery_observation);
        }
    }

    pub(crate) fn observe(&mut self, snapshot: &Snapshot) {
        if snapshot.complete() {
            self.observations
                .retain(|id, _| snapshot.devices.iter().any(|device| &device.id == id));
            let count = self.shown.len();
            self.shown
                .retain(|id| snapshot.devices.iter().any(|device| &device.id == id));
            self.dirty |= count != self.shown.len();
        }
        let available = matches!(snapshot.status, ScanStatus::Ready | ScanStatus::Partial(_));
        for (id, observation) in &mut self.observations {
            if !available || !snapshot.devices.iter().any(|device| &device.id == id) {
                observation.eligible = false;
            }
        }
        for device in &snapshot.devices {
            let observation = self.observations.entry(device.id.clone()).or_default();
            let fresh = device.battery_observation > observation.sequence;
            observation.sequence = observation.sequence.max(device.battery_observation);
            let Some(level) = device
                .valid_battery()
                .filter(|_| available && device.connection == ConnectionState::Connected)
            else {
                observation.eligible = false;
                continue;
            };
            if !fresh {
                continue;
            }
            if level > THRESHOLD {
                observation.cycle = None;
                observation.eligible = false;
                self.dirty |= self.shown.remove(&device.id);
            } else {
                if observation.cycle.is_none() {
                    self.next_cycle += 1;
                    observation.cycle = Some(self.next_cycle);
                }
                observation.eligible = self.enabled;
            }
        }
    }

    pub(crate) fn valid(&self, device: &Device, cycle: u64) -> bool {
        self.enabled
            && device.connection == ConnectionState::Connected
            && device
                .valid_battery()
                .is_some_and(|level| level <= THRESHOLD)
            && self
                .observations
                .get(&device.id)
                .is_some_and(|observation| observation.eligible && observation.cycle == Some(cycle))
    }

    pub(crate) fn candidate(&self, device: &Device) -> Option<u64> {
        let cycle = self.observations.get(&device.id)?.cycle?;
        (self.valid(device, cycle) && !self.shown.contains(&device.id)).then_some(cycle)
    }

    pub(crate) fn mark_shown(&mut self, device: &Device, cycle: u64) {
        if self.valid(device, cycle) {
            self.dirty |= self.shown.insert(device.id.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bt::BatteryStatus;

    fn reading(id: &str, level: u8, sequence: u64) -> Device {
        Device {
            id: id.into(),
            name: "同名设备".into(),
            battery: Some(level),
            connection: ConnectionState::Connected,
            battery_status: BatteryStatus::Valid,
            battery_observation: sequence,
            ..Default::default()
        }
    }

    #[test]
    fn bluetooth_low_battery_threshold_cycles_and_independent_devices() {
        for initial in [0, 19, 20, 21, 100, 101, 255] {
            let mut state = LowBattery::new(Default::default(), true);
            let device = reading("headphones", initial, 1);
            state.observe(&Snapshot::ready(1, vec![device.clone()]));
            assert_eq!(state.candidate(&device).is_some(), initial <= 20);
        }
        let mut state = LowBattery::new(Default::default(), true);
        let mut previous_cycle = None;
        for (index, level) in [20, 19, 10, 0, 21, 20].into_iter().enumerate() {
            let device = reading("headphones", level, index as u64 + 1);
            state.observe(&Snapshot::ready(index as u64 + 1, vec![device.clone()]));
            if index == 0 || index == 5 {
                let cycle = state.candidate(&device).unwrap();
                assert_ne!(previous_cycle, Some(cycle));
                state.mark_shown(&device, cycle);
                previous_cycle = Some(cycle);
            }
            assert!(state.candidate(&device).is_none());
        }
        let headphones = reading("headphones", 10, 7);
        let keyboard = reading("keyboard", 20, 7);
        state.observe(&Snapshot::ready(
            7,
            vec![headphones.clone(), keyboard.clone()],
        ));
        assert!(state.candidate(&headphones).is_none());
        assert!(state.candidate(&keyboard).is_some());
    }

    #[test]
    fn bluetooth_low_battery_invalid_reads_and_caches_cannot_trigger_or_recover() {
        let mut state = LowBattery::new(Default::default(), true);
        let low = reading("device", 20, 1);
        state.observe(&Snapshot::ready(1, vec![low.clone()]));
        state.mark_shown(&low, state.candidate(&low).unwrap());
        let mut invalid = vec![reading("device", 80, 1), reading("device", 101, 2)];
        for connection in [ConnectionState::Disconnected, ConnectionState::Unknown] {
            invalid.push(Device {
                connection,
                ..reading("device", 80, 3)
            });
        }
        for battery_status in [BatteryStatus::Failed, BatteryStatus::Unknown] {
            invalid.push(Device {
                battery_status,
                ..reading("device", 80, 4)
            });
        }
        invalid.push(Device {
            battery: None,
            ..reading("device", 80, 5)
        });
        for device in invalid {
            state.observe(&Snapshot::ready(2, vec![device.clone()]));
            assert!(state.shown().contains("device"));
            assert!(state.candidate(&device).is_none());
        }
        let high = reading("device", 80, 6);
        state.observe(&Snapshot {
            status: ScanStatus::Unavailable("蓝牙关闭".into()),
            ..Snapshot::ready(3, vec![high.clone()])
        });
        assert!(state.shown().contains("device"));
        state.observe(&Snapshot::ready(4, vec![high]));
        assert!(
            state.shown().contains("device"),
            "失败期间复用的观测不能恢复周期"
        );
        state.observe(&Snapshot::ready(5, vec![reading("device", 21, 7)]));
        assert!(state.shown().is_empty());
    }

    #[test]
    fn bluetooth_low_battery_cancel_requires_fresh_read_and_reenable_sets_baseline() {
        let mut state = LowBattery::new(Default::default(), true);
        let low = reading("device", 20, 1);
        let snapshot = Snapshot::ready(1, vec![low.clone()]);
        state.observe(&snapshot);
        let cycle = state.candidate(&low).unwrap();
        state.observe(&snapshot);
        assert!(
            state.valid(&low, cycle),
            "正常缓存间隔保留已入队提醒的展示资格"
        );
        state.observe(&Snapshot::ready(
            2,
            vec![Device {
                battery_status: BatteryStatus::Failed,
                ..low.clone()
            }],
        ));
        state.observe(&snapshot);
        assert!(state.candidate(&low).is_none());
        let fresh = reading("device", 19, 2);
        state.observe(&Snapshot::ready(3, vec![fresh.clone()]));
        assert_eq!(state.candidate(&fresh), Some(cycle));
        state.set_enabled(false, &snapshot);
        state.set_enabled(true, &Snapshot::ready(3, vec![fresh.clone()]));
        state.observe(&Snapshot::ready(3, vec![fresh.clone()]));
        assert!(state.candidate(&fresh).is_none());
        let fresh = reading("device", 19, 3);
        state.observe(&Snapshot::ready(4, vec![fresh.clone()]));
        assert_eq!(state.candidate(&fresh), Some(cycle));
    }

    #[test]
    fn bluetooth_low_battery_restart_removal_and_muted_recovery() {
        let mut state = LowBattery::new(["device".into()].into(), false);
        let low = reading("device", 10, 1);
        state.observe(&Snapshot {
            status: ScanStatus::Partial("BLE 失败".into()),
            ..Snapshot::ready(1, vec![])
        });
        assert!(state.shown().contains("device"));
        state.observe(&Snapshot::ready(2, vec![low.clone()]));
        state.reset_service();
        state.set_enabled(true, &Snapshot::default());
        state.observe(&Snapshot::ready(1, vec![low.clone()]));
        assert!(state.candidate(&low).is_none());
        state.set_enabled(false, &Snapshot::ready(1, vec![low.clone()]));
        state.observe(&Snapshot::ready(2, vec![reading("device", 21, 2)]));
        assert!(state.shown().is_empty(), "关闭提醒期间仍消费有效恢复读数");
        assert!(state.take_dirty());
        state.set_enabled(true, &Snapshot::default());
        let fresh = reading("device", 10, 3);
        state.observe(&Snapshot::ready(3, vec![fresh.clone()]));
        state.mark_shown(&fresh, state.candidate(&fresh).unwrap());
        state.observe(&Snapshot::ready(4, vec![]));
        assert!(state.shown().is_empty(), "完整枚举确认移除后清理持久化记录");
    }
}
