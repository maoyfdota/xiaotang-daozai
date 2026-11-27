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
        battery_read_at: battery.map(|_| std::time::Instant::now()),
        battery_source: battery.map(|_| crate::bt::BatterySource::SystemReport),
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
        ("7,6,5,4,3,2,1,0", [7, 6, 5, 4, 3, 2, 1, 0, 8, 9, 10]),
        ("8,7,6,5,4,3,2,1,0", [8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 10]),
    ] {
        std::fs::write(
            &path,
            format!("page_order={old}\nvisible_pages=101010101\nbluetooth_notifications=0\n"),
        )
        .unwrap();
        let settings = crate::model::Settings::load();
        assert_eq!(settings.page_order.as_slice(), expected.as_slice());
        assert_eq!(
            &settings.visible_pages[..9],
            &[true, false, true, false, true, false, true, false, true]
        );
        assert!(settings.visible_pages[9]);
        assert_eq!(settings.visible_pages.get(10).copied(), Some(true));
        assert!(!settings.bluetooth_notifications);
        settings.save();
        assert_eq!(crate::model::Settings::load().page_order.as_slice(), expected.as_slice());
    }
    drop(h);
}

#[test]
fn performance_page_configuration_migrates_old_lengths_and_round_trips_eleven_entries() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let h = H::new();
    let path = crate::model::config_dir().join("settings.txt");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    for (order, visible, expected_order) in [
        ("7,6,5,4,3,2,1,0", "00100000", vec![7,6,5,4,3,2,1,0,8,9,10]),
        ("8,7,6,5,4,3,2,1,0", "001000000", vec![8,7,6,5,4,3,2,1,0,9,10]),
        ("9,8,7,6,5,4,3,2,1,0", "0010000000", vec![9,8,7,6,5,4,3,2,1,0,10]),
    ] {
        std::fs::write(&path, format!("page_order={order}\nvisible_pages={visible}\n")).unwrap();
        let settings = crate::model::Settings::load();
        assert_eq!(settings.page_order.as_slice(), expected_order.as_slice());
        assert_eq!(&settings.visible_pages[..visible.len()], &visible.bytes().map(|b| b == b'1').collect::<Vec<_>>());
        assert_eq!(settings.visible_pages.get(10).copied(), Some(true), "旧配置缺少的性能页默认可见");
    }

    let order = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let visible = [false, true, false, true, false, true, false, true, false, true, true];
    std::fs::write(&path, "page_order=10,9,8,7,6,5,4,3,2,1,0\nvisible_pages=01010101011\n").unwrap();
    let settings = crate::model::Settings::load();
    assert_eq!(settings.page_order.as_slice(), order.as_slice());
    assert_eq!(settings.visible_pages.as_slice(), visible.as_slice());
    settings.save();
    let reloaded = crate::model::Settings::load();
    assert_eq!(reloaded.page_order.as_slice(), order.as_slice());
    assert_eq!(reloaded.visible_pages.as_slice(), visible.as_slice());
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
fn bluetooth_detail_navigation_keeps_control_and_marks_historical_battery() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    let mut value = device("detail", DeviceKind::Headphones, ConnectionState::Connected, Some(75), 1);
    value.battery_read_at = Some(std::time::Instant::now() - std::time::Duration::from_secs(120));
    update(&mut h, 1, vec![value]);
    h.app.show_page(crate::app::BLUETOOTH_PAGE);
    h.hover(H::local(200.0, 90.0), 1.0);
    h.click(H::local(100.0, 90.0));
    h.frame(vec![]);
    assert_eq!(h.app.bluetooth_ui.selected.as_deref(), Some("detail"));
    assert!(h.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.text().contains("75% · 旧值"))));
    assert!(h.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.text().contains("系统报告可能有延迟"))));
    assert!(!h.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.text().contains("充电"))));
    h.cmds.clear();
    h.click(H::local(344.0, 153.0));
    assert!(h.cmds.iter().any(|c| matches!(c, Cmd::BluetoothControl(r) if r.device_id=="detail" && r.action==Action::Disconnect)));
    h.click(H::local(72.0, 65.0));
    assert!(h.app.bluetooth_ui.selected.is_none());
}

#[test]
fn bluetooth_permission_actions_remain_available_with_cached_devices() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.managed_bt = Some(Snapshot { version: 1, status: ScanStatus::PermissionRequired("denied".into()),
        devices: vec![device("cached", DeviceKind::Keyboard, ConnectionState::Unknown, Some(80), 1)] });
    h.app.show_page(crate::app::BLUETOOTH_PAGE);
    h.hover(H::local(200.0, 90.0), 1.0);
    h.cmds.clear();
    h.click(H::local(260.0, 165.0));
    assert!(h.cmds.iter().any(|c| matches!(c, Cmd::OpenBluetoothPrivacy)));
}

#[test]
fn wireless_resident_selects_stable_lowest_values_and_preserves_custom_choices() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    let values = vec![device("C", DeviceKind::Mouse, ConnectionState::Connected, Some(75), 1),
        device("B", DeviceKind::Keyboard, ConnectionState::Connected, Some(30), 1),
        device("A", DeviceKind::Headphones, ConnectionState::Connected, Some(30), 1),
        device("unknown", DeviceKind::Mouse, ConnectionState::Connected, None, 1)];
    update(&mut h, 1, values.clone());
    assert!(h.app.wireless_resident_devices().is_empty());
    h.app.settings.wireless_resident = true;
    let ids = |app: &App| app.wireless_resident_devices().into_iter().map(|d| d.id).collect::<Vec<_>>();
    assert_eq!(ids(&h.app), ["A", "B"]);
    h.app.settings.wireless_resident_custom = true;
    h.app.settings.wireless_resident_ids = vec!["C".into(), "unknown".into()];
    assert_eq!(ids(&h.app), ["C", "unknown"]);
    let mut disconnected = values.clone();
    disconnected[0].connection = ConnectionState::Disconnected;
    update(&mut h, 2, disconnected);
    assert_eq!(ids(&h.app), ["unknown"]);
    assert_eq!(h.app.settings.wireless_resident_ids, ["C", "unknown"]);
    update(&mut h, 3, values);
    assert_eq!(ids(&h.app), ["C", "unknown"]);
    h.app.settings.wireless_resident_ring_only = true;
    h.app.settings.save();
    let loaded = crate::model::Settings::load();
    assert!(loaded.wireless_resident && loaded.wireless_resident_custom && loaded.wireless_resident_ring_only);
    assert_eq!(loaded.wireless_resident_ids, ["C", "unknown"]);
    let path = crate::model::config_dir().join("settings.txt");
    std::fs::write(&path, "resident_order=7,6,2,0,5,9,1\nwireless_resident_ids=[\"A\",\"A\",\"B\",\"C\"]\n").unwrap();
    let loaded = crate::model::Settings::load();
    assert_eq!(loaded.resident_order, [7,6,2,0,5,9,1,10]);
    assert_eq!(loaded.wireless_resident_ids, ["A", "B"]);
    assert!(!loaded.wireless_resident);
}

#[test]
fn wireless_resident_auto_keeps_connected_devices_when_battery_is_unknown() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.settings.wireless_resident = true;
    update(
        &mut h,
        1,
        vec![
            device("M240", DeviceKind::Mouse, ConnectionState::Connected, None, 1),
            device("REDMI", DeviceKind::Headphones, ConnectionState::Connected, None, 1),
        ],
    );

    let ids = h
        .app
        .wireless_resident_devices()
        .into_iter()
        .map(|device| device.id)
        .collect::<Vec<_>>();
    assert_eq!(ids, ["M240", "REDMI"]);
}

#[test]
fn wireless_side_resident_obeys_order_and_opens_device_detail() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.settings.bluetooth_low_battery_notifications = false;
    h.app.settings.wireless_resident = true;
    h.app.settings.resident_order = crate::bar_layout::DEFAULT_ORDER.to_vec();
    h.app.settings.resident_order.rotate_right(1);
    update(&mut h, 1, vec![device("mouse", DeviceKind::Mouse, ConnectionState::Connected, Some(35), 1)]);
    assert_eq!(h.app.side_items().first(), Some(&crate::side_dock::SideItem::Wireless(0)));
    for _ in 0..80 { h.frame(vec![]); }
    let point = h.shapes.iter().find_map(|shape| match &shape.shape {
        egui::Shape::Text(text) if text.galley.text()=="35" => Some(text.visual_bounding_rect().center()),
        _ => None,
    }).expect("wireless battery is visible on the collapsed side bar");
    click_side_canvas(&mut h, point);
    assert_eq!(h.app.page(), crate::app::BLUETOOTH_PAGE);
    assert_eq!(h.app.bluetooth_ui.selected.as_deref(), Some("mouse"));
}

#[test]
fn wireless_settings_render_available_and_saved_devices_without_overlapping() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.settings.wireless_resident = true;
    h.app.settings.wireless_resident_custom = true;
    h.app.settings.wireless_resident_ids = vec!["absent".into(), "mouse".into()];
    update(&mut h, 1, vec![device("mouse", DeviceKind::Mouse, ConnectionState::Connected, Some(50), 1)]);
    for dark in [false, true] {
        h.ctx.set_visuals(if dark { egui::Visuals::dark() } else { egui::Visuals::light() });
        let output = h.ctx.run(egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(620.0,500.0))), ..Default::default() }, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                h.app.wireless_resident_settings(ui);
                h.app.resident_order_settings(ui);
            });
        });
        for phrase in ["无线设备常驻", "环＋数字", "仅环", "当前不可用"] {
            assert!(output.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Text(t) if t.galley.text().contains(phrase))), "missing {phrase}");
        }
        let preview = crate::resident_settings::preview_layout(&[crate::bar_layout::WIRELESS, crate::bar_layout::CLOCK], 0);
        assert!(preview.x[crate::bar_layout::WIRELESS] < preview.x[crate::bar_layout::CLOCK]);
    }
}

#[test]
fn wireless_resident_restore_default_keeps_unavailable_slots_out_of_preview() {
    let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut h = H::new();
    h.app.settings.resident_order = crate::bar_layout::DEFAULT_ORDER.iter().copied().rev().collect();
    let draw = |h: &mut H, events| {
        h.t += 1.0 / 60.0;
        h.ctx.run(egui::RawInput { time: Some(h.t), events, screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(620.0,300.0))), ..Default::default() }, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| h.app.resident_order_settings(ui));
        })
    };
    let output = draw(&mut h, vec![]);
    let point = output.shapes.iter().find_map(|s| match &s.shape {
        egui::Shape::Text(t) if t.galley.text()=="恢复默认" => Some(t.visual_bounding_rect().center()), _ => None,
    }).unwrap();
    draw(&mut h, vec![egui::Event::PointerMoved(point)]);
    for pressed in [true,false] {
        draw(&mut h, vec![egui::Event::PointerButton { pos: point, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() }]);
    }
    assert!(h.app.settings.resident_order.is_empty());
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
        for mode in ["devices", "permission", "battery", "detail", "resident", "resident-ring"] {
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
            if !mode.starts_with("resident") && mode != "battery" {
                h.app.show_page(crate::app::BLUETOOTH_PAGE);
            }
            if mode.starts_with("resident") {
                h.app.settings.wireless_resident = true;
                h.app.settings.wireless_resident_ring_only = mode == "resident-ring";
            }
            if mode == "detail" { h.app.bluetooth_ui.selected = Some("headphones".into()); }
            let (w, hgt) = (crate::side_dock::SIDE_W, crate::side_dock::SIDE_H);
            let mut raster =
                crate::preview_raster::Raster::new((w * 2.0) as usize, (hgt * 2.0) as usize);
            for frame in 0..180 {
                h.t += 1.0 / 60.0;
                let cursor = if mode == "battery" || mode.starts_with("resident") {
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
