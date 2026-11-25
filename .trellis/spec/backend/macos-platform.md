# macOS Platform Contract

## 1. Scope / Trigger

This contract applies to the Apple Silicon macOS application target. It records the platform boundary introduced for the first macOS release, including local persistence, native display state, feature availability, rendering, and app packaging.

The first release targets `aarch64-apple-darwin` with macOS 11.0 as its deployment target. It does not claim Intel or macOS 11 runtime coverage.

## 2. Signatures

- `platform::page_available(page: usize) -> bool` owns the target-specific page capability profile.
- `model::config_dir() -> PathBuf` resolves the application data directory.
- `macos_shell::connected() -> Vec<display_layout::Display>` enumerates AppKit screens.
- `macos_shell::cursor_logical() -> Option<(f32, f32)>` samples the global pointer in logical desktop coordinates.
- `macos_shell::logical_position(display_id: &str, physical: (i32, i32)) -> Option<egui::Pos2>` converts a physical display coordinate into the logical origin expected by the window viewport.
- `display_layout::side_origin(bounds, dock, canvas, half_extent, scale, offset) -> Option<(i32, i32)>` computes a bounded side-canvas origin in physical pixels.
- `scripts/package-macos.sh` builds the arm64 release executable and creates `target/aarch64-apple-darwin/release/Daozai.app`.

## 3. Contracts

### Capabilities

- macOS exposes focus, water/sit reminders, to-do, and Open-Meteo weather.
- macOS hides Music, Shelf, Mail, AI quota, Calendar, QWeather credentials, Windows notification delivery, Bluetooth, Windows system status, startup, and foreground-window avoidance controls.
- `page_available` is the source of truth for navigation, settings, direct page entry, reminders, and optional service startup.
- Hidden persisted values remain serialized and unchanged. Startup must not migrate or initialize hidden feature settings or credentials.
- The local-reminder master switch remains available for supported local reminders.

### Persistence

- `DAOZAI_DATA_DIR`, when set, overrides the default directory on every platform.
- macOS defaults to `~/Library/Application Support/Daozai`.
- Existing file names, formats, and atomic-write behavior remain stable.

### Display and interaction

- The root viewport uses the `SIDE_W × SIDE_H` canvas and forces `Dock::Right` on macOS.
- AppKit screen and pointer APIs return logical-point coordinates. `display_layout::Display.bounds` and `work_top` use physical pixels. Convert only at the `macos_shell` boundary using the screen backing scale factor.
- Place the island within the visible frame and safe top inset when available. Use `visibleFrame` when safe-area metadata is unavailable.
- While collapsed, the transparent root viewport passes pointer events through. Turn passthrough off while the island captures the pointer and restore it after collapse. The independent settings viewport remains interactive.

### Rendering and packaging

- WGPU Metal is the primary macOS renderer. OpenGL is the fallback.
- The package script sets `MACOSX_DEPLOYMENT_TARGET=11.0`, packages the executable and Finder icon, and writes `LSMinimumSystemVersion=11.0`.
- Each `iconset` image scales the logo to 80% of its square canvas before transparent padding. `theme::app_icon()` uses the same proportion in its cached 128-pixel canvas on macOS. eframe passes this runtime image to `NSApplication::setApplicationIconImage`, overriding the bundle icon after launch, so both paths must retain the padding.
- The local-validation bundle is unsigned. Signing, notarization, DMG distribution, updates, and Intel builds remain outside this contract.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| `DAOZAI_DATA_DIR` is set | Use the override for settings and user data. |
| `DAOZAI_DATA_DIR` is unset on macOS | Resolve `~/Library/Application Support/Daozai`. |
| A saved page is unavailable on macOS | Select the first available saved page, then the first available default page. |
| A direct command targets a hidden page | Ignore or reject the request before opening its settings or starting its service. |
| `NSScreen.safeAreaInsets` is unavailable | Use the screen visible frame for the safe top boundary. |
| Display metadata or a valid viewport origin cannot be resolved | Keep the startup viewport visible so the process does not remain permanently hidden. |
| Metal initialization fails | Retry with OpenGL and report both initialization errors if fallback also fails. |
| Package tools are unavailable or the host is not macOS | Stop packaging with an error before replacing the existing `.app`. |

## 5. Good / Base / Bad Cases

- **Good**: A saved Windows configuration enables every page. macOS displays only supported pages, leaves the hidden values untouched, and does not start hidden services.
- **Base**: No page preference exists. The first available macOS page is selected and the island is placed on the primary display's right edge, vertically centered in its safe work area.
- **Bad**: `--calendar` is supplied on macOS. Calendar remains unavailable, its credentials are not loaded, and no calendar service is created.
- **Bad**: A test sets `DAOZAI_DATA_DIR`. The application uses that directory rather than reading or writing the user's default data directory.

## 6. Tests Required

- Capability tests assert the exact available page set and hidden settings contract for macOS.
- Startup and navigation tests assert stale hidden page indices and direct commands cannot open hidden pages or start their services.
- Data-path tests assert both the environment override and macOS default path. Persistence tests round-trip settings without dropping hidden legacy values.
- Display-layout tests cover right-edge placement, vertical center, offsets, Retina scale factors, invalid bounds, and preservation of the expanded panel within the safe work area.
- Bundle validation asserts the executable is arm64, `Info.plist` is valid, `LSMinimumSystemVersion` is 11.0, and the Mach-O build version has `minos 11.0`.
- Dock icon regression tests exercise `theme::app_icon()` and verify transparent margins, centered artwork, and an approximately 80% artwork width. Compare the running app's Dock icon after restart as well as inspecting the packaged ICNS.
- Runtime checks on an available macOS host verify the visible island, placement, expansion/collapse, pointer passthrough, settings interaction, and data persistence across restart. A build-target check does not substitute for runtime coverage on macOS 11.

## 7. Wrong vs Correct

### Wrong

```rust
if settings.visible_pages[page] {
    start_optional_service(page);
}
```

This treats persisted visibility as platform capability and can start a hidden Windows feature on macOS.

### Correct

```rust
if platform::page_available(page) && settings.visible_pages[page] {
    start_optional_service(page);
}
```

Use the same capability check for navigation, settings visibility, reminders, and service reconciliation so an unavailable page cannot be reached through a secondary entry point.
