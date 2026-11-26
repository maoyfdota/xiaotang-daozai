//! 蓝牙设备页面，使用后台快照展示设备，系统操作由外壳执行。

use crate::app::{App, Cmd};
use crate::bt::{
    Action, Capability, ConnectionState, Device, DeviceKind, Event, OperationStatus, Request,
    ScanStatus,
};
use std::collections::BTreeMap;

#[derive(Default)]
pub(crate) struct State {
    pub pending: BTreeMap<String, Request>,
    pub message: Option<String>,
    pub confirmation: Option<String>,
    pub selected: Option<String>,
    next_request: u64,
}
use crate::paint::{self, Card, H};
use crate::theme::{self, W};
use eframe::egui::{self, Color32, Pos2, Rect};

pub(crate) const LIST_BOUNDS: Rect = Rect {
    min: Pos2::new(42.0, 52.0),
    max: Pos2::new(377.0, 208.0),
};

impl App {
    pub(crate) fn wireless_resident_settings(&mut self, ui: &mut egui::Ui) {
        crate::extras::setting_row(ui, "无线设备常驻", "在收起的岛上显示设备电量，点击进入详情", &mut self.settings.wireless_resident);
        ui.add_enabled_ui(self.settings.wireless_resident, |ui| {
            ui.horizontal(|ui| {
                ui.label("选择设备");
                ui.selectable_value(&mut self.settings.wireless_resident_custom, false, "自动");
                ui.selectable_value(&mut self.settings.wireless_resident_custom, true, "自选");
            });
            ui.horizontal(|ui| {
                ui.label("显示样式");
                ui.selectable_value(&mut self.settings.wireless_resident_ring_only, false, "环＋数字");
                ui.selectable_value(&mut self.settings.wireless_resident_ring_only, true, "仅环");
            });
            if self.settings.wireless_resident_custom {
                ui.weak("最多两台，按选择顺序显示；断开后保留选择，重新连接自动恢复。");
                let mut devices = self.bluetooth.devices.clone();
                for id in &self.settings.wireless_resident_ids {
                    if !devices.iter().any(|d| d.id == *id) {
                        devices.push(Device { id: id.clone(), name: "已保存设备（当前不可用）".into(), ..Default::default() });
                    }
                }
                devices.sort_by(|a,b| a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)));
                if devices.is_empty() { ui.weak("配对设备读取成功后即可选择。"); }
                for device in devices {
                    let mut selected = self.settings.wireless_resident_ids.contains(&device.id);
                    let available = selected || self.settings.wireless_resident_ids.len() < 2;
                    let label = format!("{} · {}", display_name(&device), connection_label(&device, false));
                    let response = ui.push_id(&device.id, |ui| ui.add_enabled(available, egui::Checkbox::new(&mut selected, label))).inner;
                    let response = response.on_hover_text(format!("设备标识 {}", device.id));
                    if response.changed() {
                        self.settings.wireless_resident_ids.retain(|id| *id != device.id);
                        if selected { self.settings.wireless_resident_ids.push(device.id); }
                    }
                }
            } else {
                ui.weak("自动显示已连接且电量有效的设备，电量较低的优先，最多两台。");
            }
        });
    }

    pub(crate) fn wireless_resident_devices(&self) -> Vec<Device> {
        if !self.settings.wireless_resident || !matches!(self.bluetooth.status, ScanStatus::Ready | ScanStatus::Partial(_)) {
            return Vec::new();
        }
        if self.settings.wireless_resident_custom {
            let mut selected = Vec::new();
            for id in &self.settings.wireless_resident_ids {
                if selected.iter().any(|d: &Device| d.id == *id) { continue; }
                if let Some(device) = self.bluetooth.devices.iter().find(|d| d.id == *id && d.connection == ConnectionState::Connected) {
                    selected.push(device.clone());
                    if selected.len() == 2 { break; }
                }
            }
            selected
        } else {
            let now = std::time::Instant::now();
            let mut devices: Vec<_> = self.bluetooth.devices.iter()
                .filter(|d| d.connection == ConnectionState::Connected && d.valid_battery_at(now).is_some()).cloned().collect();
            devices.sort_by(|a, b| a.valid_battery_at(now).cmp(&b.valid_battery_at(now)).then_with(|| a.id.cmp(&b.id)));
            devices.truncate(2);
            devices
        }
    }

    pub(crate) fn draw_wireless_resident(&mut self, c: &Card, x: f32, y: f32, device: &Device) {
        battery_ring(c, device, x, y, 10.5, 2.0, true);
        if self.settings.wireless_resident_ring_only {
            c.circle(x, y, 7.5, theme::BLUE);
            crate::extras::device_icon(c, device.kind, x - 6.0, y - 6.0, 12.0);
        } else {
            let text = device.valid_battery().map_or_else(|| "—".into(), |level| level.to_string());
            c.centered_text(x, y, &text, 8.0, W::Semibold, theme::WHITE, H::Center);
        }
        let hint = format!("{} · {}\n点击查看详情", display_name(device), battery_label(device, false));
        if c.hit(x - 14.0, y - 14.0, 28.0, 28.0, ("wireless-resident", &device.id))
            .on_hover_text(hint).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            self.bluetooth_ui.selected = Some(device.id.clone());
            self.bluetooth_ui.confirmation = None;
            self.show_page(crate::app::BLUETOOTH_PAGE);
        }
    }

    pub fn receive_bluetooth_event(&mut self, event: Event) {
        match event {
            Event::Snapshot(snapshot) => self.receive_bluetooth(&snapshot),
            Event::Operation(result) => {
                let current = self.bluetooth_ui.pending.get(&result.request.device_id);
                if current != Some(&result.request) {
                    return;
                }
                let message = match result.status {
                    OperationStatus::Accepted => return,
                    OperationStatus::Succeeded => match result.request.action {
                        Action::Connect => "设备已连接".into(),
                        Action::Disconnect => "设备已断开".into(),
                    },
                    OperationStatus::Failed(error) => error,
                    OperationStatus::TimedOut => "操作超时，请检查设备或打开 Mac 蓝牙设置".into(),
                };
                self.bluetooth_ui.pending.remove(&result.request.device_id);
                self.bluetooth_ui.message = Some(message);
            }
        }
    }
    pub(crate) fn bluetooth_request(&mut self, id: &str, action: Action) -> Option<Cmd> {
        if self.bluetooth_ui.pending.contains_key(id) {
            return None;
        }
        if !matches!(
            self.bluetooth.status,
            ScanStatus::Ready | ScanStatus::Partial(_)
        ) {
            return None;
        }
        let device = self.bluetooth.devices.iter().find(|d| d.id == id)?;
        if device.connection == ConnectionState::Unknown {
            return None;
        }
        let capability = match action {
            Action::Connect => &device.connect,
            Action::Disconnect => &device.disconnect,
        };
        if *capability != Capability::Available {
            return None;
        }
        self.bluetooth_ui.next_request += 1;
        let request = Request {
            id: self.bluetooth_ui.next_request,
            device_id: id.into(),
            action,
        };
        self.bluetooth_ui.pending.insert(id.into(), request.clone());
        self.bluetooth_ui.message = None;
        Some(Cmd::BluetoothControl(request))
    }
    pub(crate) fn page_bluetooth(&mut self, c: &Card, cmds: &mut Vec<Cmd>) {
        let (refresh, button) = paint::capsule(
            c,
            285.0,
            7.0,
            62.0,
            22.0,
            "bluetooth-refresh",
            theme::BTN,
            theme::BTN_HOVER,
        );
        button.centered_text(
            316.0,
            18.0,
            "刷新",
            11.0,
            W::Semibold,
            theme::WHITE,
            H::Center,
        );
        if refresh.clicked() {
            cmds.push(Cmd::BluetoothRefresh);
        }

        let (settings, button) = paint::capsule(
            c,
            42.0,
            215.0,
            335.0,
            21.0,
            "bluetooth-settings",
            theme::BTN,
            theme::BTN_HOVER,
        );
        button.centered_text(
            209.5,
            225.5,
            "打开 Mac 蓝牙设置",
            10.5,
            W::Semibold,
            theme::WHITE,
            H::Center,
        );
        if settings.clicked() {
            cmds.push(Cmd::OpenBluetoothSettings);
        }

        let status = match &self.bluetooth.status {
            ScanStatus::Loading => "正在读取已配对设备…".to_owned(),
            ScanStatus::Ready => format!("已配对设备 · {} 台", self.bluetooth.devices.len()),
            ScanStatus::Partial(_) => "部分设备状态更新失败".to_owned(),
            ScanStatus::Unavailable(_) => "蓝牙暂不可用".to_owned(),
            ScanStatus::PermissionRequired(_) => "需要蓝牙访问权限".to_owned(),
        };
        let status = self.bluetooth_ui.message.as_ref().unwrap_or(&status);
        c.text(
            42.0,
            43.0,
            &c.fit(status, 10.0, W::Regular, 335.0),
            10.0,
            W::Regular,
            theme::GRAY,
            H::Left,
        );
        if let ScanStatus::Partial(error)
        | ScanStatus::Unavailable(error)
        | ScanStatus::PermissionRequired(error) = &self.bluetooth.status
        {
            c.hit(42.0, 30.0, 335.0, 20.0, "bluetooth-status")
                .on_hover_text(error);
        }

        if self.bluetooth.devices.is_empty() || matches!(self.bluetooth.status, ScanStatus::PermissionRequired(_)) {
            let (title, detail) = match &self.bluetooth.status {
                ScanStatus::Loading => ("正在读取设备", "稍后自动显示已配对设备"),
                ScanStatus::Ready => ("还没有已配对设备", "可在 Mac 蓝牙设置中配对设备"),
                ScanStatus::PermissionRequired(_) => {
                    ("需要访问蓝牙", "允许后即可显示设备、连接状态与电量")
                }
                ScanStatus::Partial(error) | ScanStatus::Unavailable(error) => {
                    ("设备读取失败", error.as_str())
                }
            };
            c.centered_text(
                209.5,
                110.0,
                title,
                13.0,
                W::Semibold,
                theme::WHITE,
                H::Center,
            );
            c.centered_text(
                209.5,
                134.0,
                &c.fit(detail, 10.0, W::Regular, 310.0),
                10.0,
                W::Regular,
                theme::GRAY,
                H::Center,
            );
            c.hit(42.0, 94.0, 335.0, 60.0, "bluetooth-empty")
                .on_hover_text(detail);
            if matches!(self.bluetooth.status, ScanStatus::PermissionRequired(_)) {
                if let Some(command) = action_button(
                    c,
                    110.0,
                    155.0,
                    105.0,
                    "允许访问蓝牙",
                    "bluetooth-authorize",
                )
                .clicked()
                .then_some(Cmd::RequestBluetoothAccess)
                {
                    cmds.push(command);
                }
                if action_button(c, 224.0, 155.0, 85.0, "权限设置", "bluetooth-privacy").clicked()
                {
                    cmds.push(Cmd::OpenBluetoothPrivacy);
                }
            }
            return;
        }

        if let Some(id) = self.bluetooth_ui.selected.clone() {
            if action_button(c, 42.0, 55.0, 60.0, "返回列表", "bluetooth-back").clicked() {
                self.bluetooth_ui.selected = None;
                self.bluetooth_ui.confirmation = None;
            }
            if let Some(device) = self.bluetooth.devices.iter().find(|device| device.id == id).cloned() {
                let unavailable = !matches!(self.bluetooth.status, ScanStatus::Ready | ScanStatus::Partial(_));
                let name = display_name(&device);
                c.text(112.0, 69.0, &c.fit(name, 12.0, W::Semibold, 265.0), 12.0, W::Semibold, theme::WHITE, H::Left);
                c.hit(112.0, 52.0, 265.0, 24.0, "bluetooth-detail-name").on_hover_text(name);
                battery_ring(c, &device, 83.0, 113.0, 28.0, 3.0, !unavailable);
                c.circle(83.0, 113.0, 22.0, theme::BLUE);
                crate::extras::device_icon(c, device.kind, 68.0, 98.0, 30.0);
                c.text(128.0, 111.0, &battery_label(&device, unavailable), 21.0, W::Semibold, theme::WHITE, H::Left);
                c.text(128.0, 133.0, connection_label(&device, unavailable), 10.5, W::Regular, theme::GRAY, H::Left);
                let row = c.shifted(42.0, 115.0);
                if let Some(action) = device_controls(&row, &device, 335.0, unavailable, &mut self.bluetooth_ui) {
                    if let Some(command) = self.bluetooth_request(&id, action) { cmds.push(command); }
                }
                c.text(42.0, 179.0, &reading_description(&device), 10.0, W::Regular, theme::GRAY, H::Left);
                c.text(42.0, 196.0, "系统报告可能有延迟", 10.0, W::Regular, theme::GRAY, H::Left);
                c.hit(42.0, 164.0, 335.0, 38.0, "bluetooth-detail-source")
                    .on_hover_text(format!("设备标识 {}\n读取时间不代表设备内部采样时间。", device.id));
            } else {
                c.centered_text(209.5, 124.0, "设备已不在列表", 13.0, W::Semibold, theme::WHITE, H::Center);
            }
            return;
        }

        let mut devices: Vec<_> = self.bluetooth.devices.to_vec();
        devices.sort_by(|a, b| {
            connection_order(a.connection)
                .cmp(&connection_order(b.connection))
                .then_with(|| a.name.cmp(&b.name))
                .then_with(|| a.id.cmp(&b.id))
        });
        let bounds = c.rect(
            LIST_BOUNDS.left(),
            LIST_BOUNDS.top(),
            LIST_BOUNDS.width(),
            LIST_BOUNDS.height(),
        );
        let mut content = egui::Ui::new(
            c.ui.ctx().clone(),
            egui::Id::new(("bluetooth-list", c.live)),
            egui::UiBuilder::new()
                .max_rect(bounds)
                .layer_id(c.painter.layer_id())
                .style(c.ui.style().clone()),
        );
        content.set_clip_rect(bounds.intersect(c.painter.clip_rect()));
        content.set_opacity(c.painter.opacity());
        if !c.live {
            content.disable();
        }
        content.spacing_mut().item_spacing.y = 0.0;
        content.spacing_mut().scroll.floating = false;
        content.spacing_mut().scroll.bar_width = 4.0 * c.s;
        let unavailable = !matches!(
            self.bluetooth.status,
            ScanStatus::Ready | ScanStatus::Partial(_)
        );
        let mut selected = None;
        let scroll = egui::ScrollArea::vertical()
            .id_salt(("bluetooth-list-scroll", c.live))
            .max_height(bounds.height())
            .auto_shrink([false, false])
            .show_rows(&mut content, 52.0 * c.s, devices.len(), |ui, range| {
                for index in range {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 52.0 * c.s),
                        egui::Sense::hover(),
                    );
                    let mut row = Card::new(ui, rect.min, ui.clip_rect());
                    row.s = c.s;
                    row.live = c.live;
                    if let Some(action) = device_row(
                        &row,
                        &devices[index],
                        rect.width() / c.s,
                        unavailable,
                        &mut self.bluetooth_ui,
                    ) {
                        selected = Some((devices[index].id.clone(), action));
                    }
                }
            });
        if let Some((id, action)) = selected {
            if let Some(command) = self.bluetooth_request(&id, action) {
                cmds.push(command);
            }
        }
        if c.live {
            self.content_scroll = Some((scroll.id, scroll.state.offset.y));
        }
    }
}

fn connection_order(state: ConnectionState) -> u8 {
    match state {
        ConnectionState::Connected => 0,
        ConnectionState::Disconnected => 1,
        ConnectionState::Unknown => 2,
    }
}

fn display_name(device: &Device) -> &str {
    if device.name.trim().is_empty() { "未命名蓝牙设备" } else { &device.name }
}

fn connection_label(device: &Device, unavailable: bool) -> &'static str {
    if unavailable { return "状态获取失败"; }
    match device.connection {
        ConnectionState::Connected => "已连接",
        ConnectionState::Disconnected => "未连接",
        ConnectionState::Unknown => "状态获取失败",
    }
}

fn battery_label(device: &Device, unavailable: bool) -> String {
    if unavailable || device.connection != ConnectionState::Connected { return "电量未知".into(); }
    if let Some(level) = device.valid_battery() { return format!("{level}%"); }
    match (device.battery, device.battery_status) {
        (Some(level), _) => format!("{level}% · 旧值"),
        (_, crate::bt::BatteryStatus::Failed) => "读取失败".into(),
        _ => "电量未知".into(),
    }
}

fn reading_description(device: &Device) -> String {
    let Some(at) = device.battery_read_at else {
        return if device.battery.is_some() { "电量来源未提供读取时间" } else { "尚未读取到有效电量" }.into();
    };
    let age = at.elapsed().as_secs();
    let age = if age < 60 { format!("{age} 秒前") }
        else if age < 3600 { format!("{} 分钟前", age / 60) }
        else { format!("{} 小时前", age / 3600) };
    let status = if device.battery_status == crate::bt::BatteryStatus::Failed { " · 读取失败" }
        else if device.valid_battery().is_none() { " · 已过期" } else { "" };
    format!("系统蓝牙报告 · {age}读取{status}")
}

pub(crate) fn battery_ring(c: &Card, device: &Device, x: f32, y: f32, radius: f32, width: f32, available: bool) {
    c.ring(x, y, radius, width, paint::alpha(theme::GRAY, 0.3));
    let level = available.then(|| device.valid_battery()).flatten()
        .filter(|_| device.connection == ConnectionState::Connected);
    if let Some(level) = level {
        if level > 0 {
            let start = -std::f32::consts::FRAC_PI_2;
            c.arc(x, y, radius, start, start + std::f32::consts::TAU * level as f32 / 100.0,
                width, if level <= 20 { theme::YELLOW } else { theme::GREEN });
        }
    }
}

fn device_row(
    c: &Card,
    device: &Device,
    width: f32,
    unavailable: bool,
    ui: &mut State,
) -> Option<Action> {
    let connection = if unavailable {
        ConnectionState::Unknown
    } else {
        device.connection
    };
    let state = match connection {
        ConnectionState::Connected => "已连接",
        ConnectionState::Disconnected => "未连接",
        ConnectionState::Unknown => "状态获取失败",
    };
    let kind = match device.kind {
        DeviceKind::Bone | DeviceKind::Earbuds | DeviceKind::Headphones => "耳机",
        DeviceKind::Speaker => "音箱",
        DeviceKind::Keyboard => "键盘",
        DeviceKind::Mouse => "鼠标",
        DeviceKind::Gamepad => "手柄",
        DeviceKind::Phone => "手机",
        DeviceKind::Other => "其他设备",
    };
    battery_ring(c, device, 18.0, 24.0, 16.0, 2.5, !unavailable);
    c.circle(18.0, 24.0, 12.0, theme::BLUE);
    crate::extras::device_icon(c, device.kind, 8.0, 14.0, 20.0);
    let name = if device.name.trim().is_empty() {
        "未命名蓝牙设备"
    } else {
        &device.name
    };
    c.text(
        36.0,
        14.0,
        &c.fit(name, 11.5, W::Semibold, width - 104.0),
        11.5,
        W::Semibold,
        theme::WHITE,
        H::Left,
    );
    c.text(
        36.0,
        29.0,
        &format!("{kind} · {state}"),
        10.0,
        W::Regular,
        theme::GRAY,
        H::Left,
    );
    let battery = (connection == ConnectionState::Connected)
        .then(|| device.valid_battery())
        .flatten();
    let battery_text = battery_label(device, unavailable);
    let battery_color = if battery.is_some_and(|level| level <= 20) {
        theme::YELLOW
    } else {
        theme::GRAY
    };
    c.text(
        width - 6.0,
        14.0,
        &battery_text,
        10.0,
        W::Regular,
        battery_color,
        H::Right,
    );
    if c.hit(0.0, 0.0, width - 112.0, 51.0, ("bluetooth-device", &device.id))
        .on_hover_text(format!("{name}\n查看设备详情"))
        .on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        ui.selected = Some(device.id.clone());
        ui.confirmation = None;
    }
    let selected = device_controls(c, device, width, unavailable, ui);
    c.seg((0.0, 51.5), (width, 51.5), 0.5, Color32::from_gray(42));
    selected
}

fn device_controls(c: &Card, device: &Device, width: f32, unavailable: bool, ui: &mut State) -> Option<Action> {
    let connection = if unavailable { ConnectionState::Unknown } else { device.connection };
    let action = if connection == ConnectionState::Connected {
        Action::Disconnect
    } else {
        Action::Connect
    };
    let capability = match action {
        Action::Connect => &device.connect,
        Action::Disconnect => &device.disconnect,
    };
    let pending = ui.pending.get(&device.id);
    let mut selected = None;
    if let Some(request) = pending {
        c.text(
            36.0,
            44.0,
            if request.action == Action::Connect {
                "连接中…"
            } else {
                "断开中…"
            },
            9.0,
            W::Regular,
            theme::BLUE,
            H::Left,
        );
    } else if ui.confirmation.as_deref() == Some(&device.id) {
        c.text(
            36.0,
            44.0,
            "断开键盘后将无法输入",
            9.0,
            W::Regular,
            theme::YELLOW,
            H::Left,
        );
        if action_button(
            c,
            width - 112.0,
            28.0,
            48.0,
            "取消",
            ("bluetooth-cancel", &device.id),
        )
        .clicked()
        {
            ui.confirmation = None;
        }
        if action_button(
            c,
            width - 60.0,
            28.0,
            54.0,
            "确认断开",
            ("bluetooth-confirm", &device.id),
        )
        .clicked()
        {
            ui.confirmation = None;
            if !unavailable
                && connection == ConnectionState::Connected
                && *capability == Capability::Available
            {
                selected = Some(Action::Disconnect);
            }
        }
    } else if !unavailable
        && connection != ConnectionState::Unknown
        && *capability == Capability::Available
    {
        let text = match action {
            Action::Connect => "连接",
            Action::Disconnect => "断开",
        };
        if action_button(
            c,
            width - 60.0,
            28.0,
            54.0,
            text,
            ("bluetooth-control", &device.id),
        )
        .clicked()
        {
            if action == Action::Disconnect && device.kind == DeviceKind::Keyboard {
                ui.confirmation = Some(device.id.clone());
            } else {
                selected = Some(action);
            }
        }
    } else {
        c.text(
            36.0,
            44.0,
            "请在 Mac 蓝牙设置中操作",
            9.0,
            W::Regular,
            theme::GRAY,
            H::Left,
        );
    }
    selected
}

fn action_button(
    c: &Card,
    x: f32,
    y: f32,
    width: f32,
    text: &str,
    id: impl std::hash::Hash,
) -> egui::Response {
    let response = c.hit(x, y, width, 20.0, id);
    let (hover, press) = c.hover_t(&response);
    let draw = c.scaled(x + width / 2.0, y + 10.0, 1.0 - 0.05 * press);
    draw.fill(
        x,
        y,
        width,
        20.0,
        10.0,
        paint::mix(theme::BTN, theme::BTN_HOVER, hover),
    );
    draw.centered_text(
        x + width / 2.0,
        y + 10.0,
        text,
        9.5,
        W::Semibold,
        theme::WHITE,
        H::Center,
    );
    response
}
