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

pub fn logical_position(display_id: &str, physical: (i32, i32)) -> Option<eframe::egui::Pos2> {
    let screen = screen_info().into_iter().find(|screen| screen.display.id == display_id)?;
    if !screen.scale_factor.is_finite() || screen.scale_factor <= 0.0 { return None; }
    Some(eframe::egui::pos2(physical.0 as f32 / screen.scale_factor as f32, physical.1 as f32 / screen.scale_factor as f32))
}

