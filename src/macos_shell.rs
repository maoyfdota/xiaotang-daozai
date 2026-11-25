//! AppKit screen metadata and pointer sampling used by the macOS window shell.

use objc::{class, msg_send, sel, sel_impl, runtime::Object};
use std::ffi::{c_char, CStr};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Point { x: f64, y: f64 }

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Size { width: f64, height: f64 }

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Rect { origin: Point, size: Size }

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct EdgeInsets { top: f64, left: f64, bottom: f64, right: f64 }

struct ScreenInfo {
    display: daozai::display_layout::Display,
    scale_factor: f64,
    frame: Rect,
}

fn screen_info() -> Vec<ScreenInfo> {
    unsafe {
        let screens: *mut Object = msg_send![class!(NSScreen), screens];
        if screens.is_null() { return Vec::new(); }
        let count: usize = msg_send![screens, count];
        let main_screen: *mut Object = msg_send![class!(NSScreen), mainScreen];
        let mut raw = Vec::with_capacity(count);
        let mut virtual_top = f64::NEG_INFINITY;
        for index in 0..count {
            let screen: *mut Object = msg_send![screens, objectAtIndex: index];
            if screen.is_null() { continue; }
            let frame: Rect = msg_send![screen, frame];
            let visible: Rect = msg_send![screen, visibleFrame];
            let scale_factor: f64 = msg_send![screen, backingScaleFactor];
            let safe_top = safe_top_inset(screen);
            let localized_name: *mut Object = msg_send![screen, localizedName];
            let label = utf8_string(localized_name).unwrap_or_else(|| "显示器".to_owned());
            let number = display_number(screen).unwrap_or(index as u32 + 1);
            let primary: bool = msg_send![screen, isEqual: main_screen];
            virtual_top = virtual_top.max(frame.origin.y + frame.size.height);
            raw.push((frame, visible, safe_top, scale_factor, number, label, primary));
        }
        if !virtual_top.is_finite() { return Vec::new(); }

        raw.into_iter().map(|(frame, visible, safe_top, scale_factor, number, name, primary)| {
            let pixels = |value: f64| (value * scale_factor).round() as i32;
            let frame_top = frame.origin.y + frame.size.height;
            let top = pixels(virtual_top - frame_top);
            let bottom = pixels(virtual_top - frame.origin.y);
            let left = pixels(frame.origin.x);
            let right = pixels(frame.origin.x + frame.size.width);
            let visible_top = visible.origin.y + visible.size.height;
            let reserved_top = (frame_top - visible_top).max(safe_top);
            let work_top = top.saturating_add(pixels(reserved_top));
            let resolution = format!("{} × {}", pixels(frame.size.width), pixels(frame.size.height));
            let display = daozai::display_layout::Display {
                id: format!("macos-{number}"),
                label: format!("显示器 {number} · {name} · {resolution}{}", if primary { " · 主屏" } else { "" }),
                bounds: daozai::display_layout::Bounds { left, top, right, bottom },
                work_top,
                primary,
            };
            ScreenInfo { display, scale_factor, frame }
        }).collect()
    }
}

unsafe fn safe_top_inset(screen: *mut Object) -> f64 {
    let responds: bool = msg_send![screen, respondsToSelector: sel!(safeAreaInsets)];
    if responds {
        let insets: EdgeInsets = msg_send![screen, safeAreaInsets];
        insets.top.max(0.0)
    } else {
        0.0
    }
}

unsafe fn display_number(screen: *mut Object) -> Option<u32> {
    let description: *mut Object = msg_send![screen, deviceDescription];
    if description.is_null() { return None; }
    let key = b"NSScreenNumber\0";
    let key: *mut Object = msg_send![class!(NSString), stringWithUTF8String: key.as_ptr() as *const c_char];
    let number: *mut Object = msg_send![description, objectForKey: key];
    if number.is_null() { return None; }
    Some(msg_send![number, unsignedIntValue])
}

unsafe fn utf8_string(value: *mut Object) -> Option<String> {
    if value.is_null() { return None; }
    let text: *const c_char = msg_send![value, UTF8String];
    if text.is_null() { None } else { Some(CStr::from_ptr(text).to_string_lossy().into_owned()) }
}

pub fn connected() -> Vec<daozai::display_layout::Display> {
    let mut displays: Vec<_> = screen_info().into_iter().map(|screen| screen.display).collect();
    displays.sort_by(|left, right| left.label.cmp(&right.label));
    displays
}

pub fn cursor_logical() -> Option<(f32, f32)> {
    unsafe {
        let screens = screen_info();
        let virtual_top = screens.iter().map(|screen| screen.frame.origin.y + screen.frame.size.height).fold(f64::NEG_INFINITY, f64::max);
        if !virtual_top.is_finite() { return None; }
        let location: Point = msg_send![class!(NSEvent), mouseLocation];
        Some((location.x as f32, (virtual_top - location.y) as f32))
    }
}

/// AppKit 桌面点先按本帧缩放换算，再减去已经缩放的 viewport 原点。
pub fn pointer_in_viewport(point: (f32, f32), origin: eframe::egui::Pos2, scale: f32) -> eframe::egui::Pos2 {
    eframe::egui::pos2(point.0 / scale - origin.x, point.1 / scale - origin.y)
}

pub fn logical_position(display_id: &str, physical: (i32, i32)) -> Option<eframe::egui::Pos2> {
    let screen = screen_info().into_iter().find(|screen| screen.display.id == display_id)?;
    if !screen.scale_factor.is_finite() || screen.scale_factor <= 0.0 { return None; }
    Some(eframe::egui::pos2(physical.0 as f32 / screen.scale_factor as f32, physical.1 as f32 / screen.scale_factor as f32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use daozai::app::{App, Env, Options};
    use eframe::egui::{self, Pos2, Rect, Vec2};

    #[test]
    fn scaled_capsule_hover_opens_panel_and_settings() {
        let data_dir = std::env::temp_dir().join(format!("daozai-macos-pointer-{}", std::process::id()));
        let previous_dir = std::env::var_os("DAOZAI_DATA_DIR");
        std::env::set_var("DAOZAI_DATA_DIR", &data_dir);
        for scale in [0.9, 1.0, 1.25] {
            for native_origin in [egui::pos2(1020.0, 318.0), egui::pos2(-800.0, -200.0)] {
                let ctx = egui::Context::default();
                let mut app = App::new(&ctx, Options { fetch_lyrics: false, pet_idle: 1e9 });
                app.settings.island_shape = daozai::model::IslandShape::Pill;
                app.settings.skin = daozai::model::Skin::Fluid;
                app.settings.island_size = (scale * 100.0) as u32;
                let canvas = Rect::from_min_size(Pos2::ZERO, egui::vec2(daozai::side_dock::SIDE_W, daozai::side_dock::SIDE_H));
                let scaled_origin = native_origin / scale;
                let mut time = 0.0;
                let mut frame = |app: &mut App, local: Pos2, pressed: Option<bool>| {
                    time += 1.0 / 60.0;
                    let native_cursor = native_origin + local.to_vec2() * scale;
                    let cursor = pointer_in_viewport((native_cursor.x, native_cursor.y), scaled_origin, scale);
                    let mut raw = egui::RawInput {
                        time: Some(time), screen_rect: Some(canvas), focused: true,
                        events: vec![egui::Event::PointerMoved(local)], ..Default::default()
                    };
                    let viewport = raw.viewports.get_mut(&egui::ViewportId::ROOT).unwrap();
                    viewport.native_pixels_per_point = Some(2.0 * scale);
                    viewport.outer_rect = Some(Rect::from_min_size(scaled_origin, canvas.size()));
                    if let Some(pressed) = pressed {
                        raw.events.push(egui::Event::PointerButton {
                            pos: local, button: egui::PointerButton::Primary, pressed, modifiers: Default::default(),
                        });
                    }
                    let env = Env { t: time, cursor: Some(cursor), clock: "09:41:00".into(), ..Default::default() };
                    let mut result = None;
                    let _ = ctx.run(raw, |ctx| {
                        egui::CentralPanel::default().frame(egui::Frame::new()).show(ctx, |ui| {
                            result = Some(app.frame(ui, canvas, &env));
                        });
                    });
                    result.unwrap()
                };
                let mut output = frame(&mut app, egui::pos2(20.0, 300.0), None);
                for _ in 0..120 {
                    output = frame(&mut app, egui::pos2(20.0, 300.0), None);
                }
                let capsule = output.hit.center();
                for _ in 0..120 {
                    output = frame(&mut app, capsule, None);
                }
                assert!(output.expanded, "capsule must expand at scale {scale}, origin {native_origin:?}");
                assert!(app.captures_root_pointer(), "expanded panel must receive clicks");
                // side_hit_rect expands the panel by (4, 2), and the gear is at (394, 18).
                let gear = output.hit.min + Vec2::new(398.0, 20.0);
                frame(&mut app, gear, None);
                frame(&mut app, gear, Some(true));
                frame(&mut app, gear, Some(false));
                assert!(app.settings_open, "settings gear must work at scale {scale}");
                for _ in 0..120 {
                    output = frame(&mut app, egui::pos2(20.0, 20.0), None);
                }
                assert!(!output.expanded);
                assert!(!app.captures_root_pointer(), "pointer must pass through after collapse");
            }
        }
        match previous_dir {
            Some(path) => std::env::set_var("DAOZAI_DATA_DIR", path),
            None => std::env::remove_var("DAOZAI_DATA_DIR"),
        }
        let _ = std::fs::remove_dir_all(data_dir);
    }
}

