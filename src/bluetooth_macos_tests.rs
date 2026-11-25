use super::*;
use crate::app::Cmd;
use crate::bt::{
    Action, BatteryStatus, Capability, ConnectionState, Device, DeviceKind, Event, OperationResult,
    OperationStatus, ScanStatus, Snapshot,
};

fn device(
    id: &str,
    kind: DeviceKind,
    connection: ConnectionState,
    battery: Option<u8>,
    observation: u64,
) -> Device {
    Device {
        id: id.into(),
        name: format!("设备 {id}"),
        kind,
        connection,
        battery,
        battery_status: if battery.is_some() {
            BatteryStatus::Valid
        } else {
            BatteryStatus::Unknown
        },
        battery_observation: observation,
        connect: Capability::Available,
        disconnect: Capability::Available,
        ..Default::default()
    }
}
fn update(h: &mut H, version: u64, devices: Vec<Device>) {
    h.managed_bt = Some(Snapshot::ready(version, devices));
    h.frame(vec![]);
}

#[test]
fn bluetooth_management_is_independent_of_reminders_and_read_failures() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.settings.bluetooth_notifications = false;
    let mut value = device(
        "A",
        DeviceKind::Headphones,
        ConnectionState::Connected,
        Some(90),
        1,
    );
    update(&mut h, 1, vec![value.clone()]);
    assert_eq!(h.app.bluetooth.devices.len(), 1);
    h.app.settings.bluetooth_notifications = true;
    h.frame(vec![]);
    assert!(h
        .app
        .alerts
        .iter()
        .all(|a| a.bluetooth_connection().is_none()));
    let mut failed = h.app.bluetooth.clone();
    failed.version = 2;
    failed.status = ScanStatus::Unavailable("test".into());
    failed.devices[0].connection = ConnectionState::Unknown;
    failed.devices[0].battery_status = BatteryStatus::Failed;
    h.managed_bt = Some(failed);
    h.frame(vec![]);
    assert!(h
        .app
        .alerts
        .iter()
        .all(|a| a.bluetooth_connection().is_none()));
    value.connection = ConnectionState::Disconnected;
    update(&mut h, 3, vec![value]);
    assert!(h
        .app
        .alerts
        .iter()
        .chain(h.app.notice.iter())
        .any(|a| a.bluetooth_connection() == Some((DeviceKind::Headphones, false))));
}

#[test]
fn bluetooth_control_ignores_late_results_and_deduplicates_requests() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    update(
        &mut h,
        1,
        vec![device(
            "A",
            DeviceKind::Headphones,
            ConnectionState::Disconnected,
            None,
            0,
        )],
    );
    let Cmd::BluetoothControl(first) = h.app.bluetooth_request("A", Action::Connect).unwrap()
    else {
        panic!()
    };
    assert!(h.app.bluetooth_request("A", Action::Connect).is_none());
    h.app
        .receive_bluetooth_event(Event::Operation(OperationResult {
            request: first.clone(),
            status: OperationStatus::TimedOut,
        }));
    let Cmd::BluetoothControl(second) = h.app.bluetooth_request("A", Action::Connect).unwrap()
    else {
        panic!()
    };
    h.app
        .receive_bluetooth_event(Event::Operation(OperationResult {
            request: first,
            status: OperationStatus::Succeeded,
        }));
    assert_eq!(h.app.bluetooth_ui.pending.get("A"), Some(&second));
    h.app
        .receive_bluetooth_event(Event::Operation(OperationResult {
            request: second,
            status: OperationStatus::Failed("offline".into()),
        }));
    assert!(h.app.bluetooth_ui.pending.is_empty());
    assert_eq!(h.app.bluetooth_ui.message.as_deref(), Some("offline"));
}

#[test]
#[cfg(target_os = "macos")]
fn bluetooth_keyboard_cancel_never_sends_disconnect_and_confirm_sends_once() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    update(
        &mut h,
        1,
        vec![device(
            "keyboard",
            DeviceKind::Keyboard,
            ConnectionState::Connected,
            Some(80),
            1,
        )],
    );
    h.app.show_page(crate::app::BLUETOOTH_PAGE);
    h.hover(H::local(200.0, 90.0), 1.0);
    h.cmds.clear();
    h.click(H::local(344.0, 90.0));
    assert_eq!(h.app.bluetooth_ui.confirmation.as_deref(), Some("keyboard"));
    assert!(!h.cmds.iter().any(|c| matches!(c, Cmd::BluetoothControl(_))));
    h.click(H::local(290.0, 90.0));
    assert!(h.app.bluetooth_ui.confirmation.is_none());
    assert!(!h.cmds.iter().any(|c| matches!(c, Cmd::BluetoothControl(_))));
    h.click(H::local(344.0, 90.0));
    h.click(H::local(344.0, 90.0));
    assert_eq!(
        h.cmds
            .iter()
            .filter(|c| matches!(c, Cmd::BluetoothControl(_)))
            .count(),
        1
    );
}

#[test]
fn bluetooth_low_battery_persists_only_after_actual_display_and_survives_restart() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    let value = device(
        "low",
        DeviceKind::Headphones,
        ConnectionState::Connected,
        Some(20),
        1,
    );
    update(&mut h, 1, vec![value.clone()]);
    for _ in 0..160 {
        h.frame(vec![]);
    }
    assert!(
        h.app.bt_low_battery.shown().contains("low"),
        "visible banner must acknowledge the cycle"
    );
    let (store, shown) = crate::model::BluetoothAlertStore::load();
    drop(store);
    assert!(shown.contains("low"));
    h.app = App::new(
        &h.ctx,
        Options {
            fetch_lyrics: false,
            pet_idle: 1e9,
        },
    );
    update(&mut h, 2, vec![value.clone()]);
    assert!(h.app.bt_low_battery.candidate(&value).is_none());
    let mut recovered = value.clone();
    recovered.battery = Some(21);
    recovered.battery_observation = 2;
    update(&mut h, 3, vec![recovered]);
    let mut low = value;
    low.battery = Some(19);
    low.battery_observation = 3;
    update(&mut h, 4, vec![low.clone()]);
    assert!(h.app.bt_low_battery.candidate(&low).is_some());
}

#[test]
fn bluetooth_pending_banner_is_cancelled_on_invalid_data_and_not_marked_shown() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.show_page(crate::app::FOCUS_PAGE);
    h.hover(H::local(200.0, 60.0), 1.0);
    let mut low = device(
        "low",
        DeviceKind::Keyboard,
        ConnectionState::Connected,
        Some(0),
        1,
    );
    update(&mut h, 1, vec![low.clone()]);
    assert!(!h.app.bt_low_battery.shown().contains("low"));
    low.battery_status = BatteryStatus::Failed;
    low.battery_observation = 2;
    update(&mut h, 2, vec![low]);
    assert!(h
        .app
        .alerts
        .iter()
        .chain(h.app.notice.iter())
        .all(|a| a.bluetooth_low_battery().is_none()));
    assert!(!h.app.bt_low_battery.shown().contains("low"));
}

#[test]
fn bluetooth_page_migration_preserves_old_permutations_and_switches() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let h = H::new();
    let path = crate::model::config_dir().join("settings.txt");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    for (old, expected) in [
        ("7,6,5,4,3,2,1,0", [7, 6, 5, 4, 3, 2, 1, 0, 8, 9]),
        ("8,7,6,5,4,3,2,1,0", [8, 7, 6, 5, 4, 3, 2, 1, 0, 9]),
    ] {
        std::fs::write(
            &path,
            format!("page_order={old}\nvisible_pages=101010101\nbluetooth_notifications=0\n"),
        )
        .unwrap();
        let settings = crate::model::Settings::load();
        assert_eq!(settings.page_order, expected);
        assert_eq!(
            &settings.visible_pages[..9],
            &[true, false, true, false, true, false, true, false, true]
        );
        assert!(settings.visible_pages[9]);
        assert!(!settings.bluetooth_notifications);
        settings.save();
        assert_eq!(crate::model::Settings::load().page_order, expected);
    }
    drop(h);
}

#[test]
#[cfg(target_os = "macos")]
fn bluetooth_permissions_have_distinct_ui_and_request_actions() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.managed_bt = Some(Snapshot {
        version: 1,
        status: ScanStatus::PermissionRequired("denied".into()),
        ..Default::default()
    });
    h.app.show_page(crate::app::BLUETOOTH_PAGE);
    h.hover(H::local(200.0, 90.0), 1.0);
    h.cmds.clear();
    h.click(H::local(160.0, 165.0));
    assert!(h
        .cmds
        .iter()
        .any(|c| matches!(c, Cmd::RequestBluetoothAccess)));
    assert!(h.app.bluetooth_request("A", Action::Connect).is_none());
}

#[test]
#[ignore = "导出 Mac 蓝牙页面与低电量提醒合成截图"]
fn bluetooth_macos_gallery() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let directory = std::path::PathBuf::from(
        std::env::var_os("DAOZAI_BLUETOOTH_PREVIEW").expect("preview directory"),
    );
    std::fs::create_dir_all(&directory).unwrap();
    for (name, skin) in [
        ("dark", crate::model::Skin::Classic),
        ("light", crate::model::Skin::Cream),
    ] {
        for mode in ["devices", "permission", "battery"] {
            let mut h = H::new();
            h.app.settings.skin = skin;
            let devices = vec![
                device(
                    "headphones",
                    DeviceKind::Headphones,
                    ConnectionState::Connected,
                    Some(if mode == "battery" { 19 } else { 80 }),
                    1,
                ),
                device(
                    "keyboard",
                    DeviceKind::Keyboard,
                    ConnectionState::Disconnected,
                    None,
                    0,
                ),
                device(
                    "mouse",
                    DeviceKind::Mouse,
                    ConnectionState::Connected,
                    Some(55),
                    1,
                ),
            ];
            let mut snapshot = Snapshot::ready(1, devices);
            if mode == "permission" {
                snapshot.devices.clear();
                snapshot.status = ScanStatus::PermissionRequired("需要访问蓝牙".into());
            }
            h.managed_bt = Some(snapshot.clone());
            if mode != "battery" {
                h.app.show_page(crate::app::BLUETOOTH_PAGE);
            }
            let (w, hgt) = (crate::side_dock::SIDE_W, crate::side_dock::SIDE_H);
            let mut raster =
                crate::preview_raster::Raster::new((w * 2.0) as usize, (hgt * 2.0) as usize);
            for frame in 0..180 {
                h.t += 1.0 / 60.0;
                let cursor = if mode == "battery" {
                    None
                } else {
                    Some(Pos2::new(w - 200.0, hgt / 2.0))
                };
                let input = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, hgt))),
                    time: Some(h.t),
                    viewports: std::iter::once((
                        egui::ViewportId::ROOT,
                        egui::ViewportInfo {
                            native_pixels_per_point: Some(2.0),
                            ..Default::default()
                        },
                    ))
                    .collect(),
                    ..Default::default()
                };
                let env = Env {
                    t: h.t,
                    clock: "09:41:00".into(),
                    cursor,
                    managed_bt: Some(snapshot.clone()),
                    ..Default::default()
                };
                let output = h.ctx.run(input, |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| {
                            h.app.frame(ui, ui.max_rect(), &env);
                        });
                });
                raster.update_textures(&output.textures_delta);
                raster.buf.fill([0.16, 0.18, 0.22, 1.0]);
                if frame == 179 {
                    raster.draw(
                        &h.ctx.tessellate(output.shapes, output.pixels_per_point),
                        output.pixels_per_point,
                    );
                }
                raster.free_textures(&output.textures_delta);
            }
            let mut rgba = Vec::new();
            raster.to_rgba8(&mut rgba);
            image::save_buffer(
                directory.join(format!("mac-bluetooth-{mode}-{name}.png")),
                &rgba,
                (w * 2.0) as u32,
                (hgt * 2.0) as u32,
                image::ColorType::Rgba8,
            )
            .unwrap();
        }
    }
}

#[test]
fn bluetooth_reminder_switches_are_independent_and_hidden_page_keeps_low_battery() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for connections in [false, true] {
        for battery in [false, true] {
            let mut h = H::new();
            h.app.settings.bluetooth_notifications = connections;
            h.app.settings.bluetooth_low_battery_notifications = battery;
            h.app.set_page_visible(crate::app::BLUETOOTH_PAGE, false);
            let low = device(
                "switches",
                DeviceKind::Headphones,
                ConnectionState::Connected,
                Some(20),
                1,
            );
            update(&mut h, 1, vec![low.clone()]);
            assert_eq!(h.app.bt_low_battery.candidate(&low).is_some(), battery);
            let mut offline = low;
            offline.connection = ConnectionState::Disconnected;
            offline.battery_status = BatteryStatus::Unknown;
            update(&mut h, 2, vec![offline]);
            assert_eq!(
                h.app
                    .alerts
                    .iter()
                    .chain(h.app.notice.iter())
                    .chain(h.app.hud_notice.iter())
                    .any(|a| a.bluetooth_connection().is_some()),
                connections
            );
            assert!(h
                .app
                .alerts
                .iter()
                .chain(h.app.notice.iter())
                .all(|a| a.bluetooth_low_battery().is_none()));
            assert_eq!(h.app.bluetooth.devices.len(), 1);
        }
    }
}

#[test]
#[cfg(target_os = "macos")]
fn bluetooth_long_list_scroll_keeps_the_page_and_reaches_last_device() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    let devices = (0..25)
        .map(|i| {
            device(
                &format!("{i:02}"),
                DeviceKind::Keyboard,
                ConnectionState::Disconnected,
                None,
                0,
            )
        })
        .collect();
    update(&mut h, 1, devices);
    h.app.show_page(crate::app::BLUETOOTH_PAGE);
    h.hover(H::local(200.0, 90.0), 1.0);
    for _ in 0..30 {
        h.frame(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: Vec2::new(0.0, -120.0),
            modifiers: Default::default(),
        }]);
    }
    for _ in 0..60 {
        h.frame(vec![]);
    }
    assert_eq!(h.app.page(), crate::app::BLUETOOTH_PAGE);
    assert!(h
        .app
        .content_scroll
        .is_some_and(|(_, offset)| offset > 1000.0));
    assert!(h.shapes.iter().any(
        |shape| matches!(&shape.shape,egui::Shape::Text(t) if t.galley.text().contains("设备 24"))
    ));
}
