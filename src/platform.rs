//! Capabilities exposed by each native application target.

/// Whether a page is available in the current platform build.
pub const fn page_available(page: usize) -> bool {
    if page >= 10 {
        return false;
    }
    if cfg!(target_os = "macos") {
        matches!(page, 1 | 2 | 4 | 8 | 9)
    } else {
        page < 9
    }
}

pub const fn local_reminder_settings_available() -> bool {
    true
}

pub const fn notification_controls_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn system_status_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn glass_blur_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn autostart_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn qweather_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn foreground_avoidance_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn renderer_compatibility_available() -> bool {
    !cfg!(target_os = "macos")
}

pub const fn application_updates_available() -> bool {
    cfg!(windows)
}

pub const fn diagnostics_collection_available() -> bool {
    cfg!(windows)
}

/// Page settings have stable indices because those indices are persisted and used by old shortcuts.
pub const fn settings_tab_available(tab: usize) -> bool {
    let page = match tab {
        1 => Some(0),
        2 => Some(1),
        3 => Some(2),
        6 => return local_reminder_settings_available(),
        4 => Some(4),
        5 => Some(3),
        7 => Some(5),
        8 => Some(7),
        11 => Some(6),
        12 => Some(8),
        15 => Some(9),
        _ => None,
    };
    match page {
        Some(page) => page_available(page),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macos_first_release_exposes_only_local_core_pages() {
        if cfg!(target_os = "macos") {
            assert_eq!((0..10).filter(|page| page_available(*page)).collect::<Vec<_>>(), [1, 2, 4, 8, 9]);
            assert!(!qweather_available());
            assert!(local_reminder_settings_available());
            assert!(!system_status_available());
            assert!(!autostart_available());
            assert!(!settings_tab_available(1));
            assert!(!settings_tab_available(5));
            assert!(settings_tab_available(6));
            assert!(!settings_tab_available(7));
            assert!(!settings_tab_available(8));
            assert!(!settings_tab_available(11));
            assert!(!notification_controls_available());
            assert!(!foreground_avoidance_available());
            assert!(!renderer_compatibility_available());
            assert!(!application_updates_available());
            assert!(!diagnostics_collection_available());
        } else {
            assert!((0..9).all(page_available));
            assert!(qweather_available());
            assert!(local_reminder_settings_available());
            assert!(system_status_available());
            assert!(autostart_available());
            assert!(notification_controls_available());
            assert!(foreground_avoidance_available());
            assert!(renderer_compatibility_available());
            assert_eq!(application_updates_available(),cfg!(windows));
            assert_eq!(diagnostics_collection_available(),cfg!(windows));
        }
        assert_eq!(page_available(9),cfg!(target_os="macos"));
        assert!(!page_available(10));
    }
}
