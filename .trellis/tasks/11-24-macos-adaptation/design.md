# Mac 适配技术设计

## Architecture

Keep the existing eframe application and shared `App` state. Add a macOS adapter around the root viewport, screen placement, pointer sampling, and rendering. This task validates the macOS first-release path. Windows-only behavior is unavailable in the Mac bundle and is outside this task's acceptance scope.

The first macOS profile exposes clock, pet appearance, local focus, water/sit reminders, to-do, and Open-Meteo weather. It hides the Music, file Shelf, Mail, AI quota, and Calendar pages, Windows notification and Bluetooth controls, battery/network/volume system-status controls, startup and foreground-window avoidance settings, system-notification controls, and QWeather credential controls. The profile must filter navigation and settings from the same source so an unavailable page cannot be reopened by a stale saved index, command-line argument, or direct command.

Keep the shared local-reminder master switch available on macOS because it controls first-release focus, water/sit, and to-do reminders. Hide the Windows notification delivery modes, system-notification sound, notification-reading controls, and chat-client settings while preserving their saved values.

## Components

### Platform capabilities

- Add `src/platform.rs` as the single owner of the per-target page and settings capability profile.
- Keep the existing `Settings.visible_pages` and `page_order` serialization schema. Apply the capability profile when initializing the current page, navigating, rendering the page manager, and starting optional background services.
- Preserve stored values for hidden pages. Do not rewrite settings data to remove unavailable features.
- Make `App::module_on` combine saved visibility with platform capability. Use it for reminders and service reconciliation as well as rendering.
- Filter direct entry points such as `open_settings_category`, `show_page`, `--calendar`, calendar commands, and module reconciliation through the same capability checks. A hidden page request must not route into that page's settings category.
- On macOS, skip `quota_initialize` and `migrate_merged_switches`. They inspect or rewrite state for hidden features, including page visibility and quota configuration. Do not construct the quota service or spawn the music poller on macOS.
- Limit the Mac weather runtime to Open-Meteo. Do not load or query QWeather credentials on this platform, and leave any saved QWeather settings untouched.
- Keep unavailable Windows system values absent in `App::Env` and hide the associated controls.
- Keep the local-reminder master switch available while filtering Windows notification controls from the shared settings tab.
- Use the macOS `open` command for visible actions that open the local data directory. The file Shelf is hidden, so its Windows Explorer reveal action remains unavailable.
- Add focused tests for the macOS profile and for navigation fallback when a saved current page is unavailable.

### Main window and AppKit adapter

- Keep the root window transparent, undecorated, fixed-size, and always on top using eframe viewport options. On macOS, use the existing `SIDE_W × SIDE_H` canvas and force `Dock::Right` so the island is placed at the display's right edge and vertically centered when its position offset is zero.
- Add a target-gated macOS module for screen safe-area and global pointer data. Use AppKit `NSScreen.safeAreaInsets` when the running macOS version exposes the property. Fall back to the visible screen frame on older supported systems.
- Position the right-docked canvas with `display_layout::side_origin`, using the selected screen's visible frame and notch safe-area insets, then convert between AppKit coordinates, physical pixels, and egui points at the platform boundary.
- Use `ViewportCommand::MousePassthrough` to let clicks reach windows below the transparent island. Sample the global pointer at a low rate while the island is collapsed, disable passthrough when the pointer reaches its activation area, and restore passthrough after the island collapses. This preserves the existing hover/expand interaction without installing a global keyboard hook.
- Keep the settings viewport as a separate normal interactive window. Mouse passthrough applies only to the root island viewport.
- Represent Windows-only network, volume, battery, and Bluetooth values as unavailable on macOS instead of reading Windows-only state fields.

### Renderer

- Use WGPU with the Metal backend for the macOS target. The locked eframe dependency enables Metal for macOS.
- Keep Glow/OpenGL as a fallback if Metal initialization fails. Do not run the Windows DX12/Vulkan fallback sequence on macOS.
- Keep the Mac renderer independent of Windows renderer ordering and compatibility settings.

### Local persistence and bundle

- Preserve `DAOZAI_DATA_DIR` as the first-priority override.
- Use `~/Library/Application Support/Daozai` as the macOS default. Keep the Windows `%APPDATA%/Daozai` path unchanged and retain existing file formats and atomic-write behavior.
- Add a macOS packaging script that builds `aarch64-apple-darwin` with `MACOSX_DEPLOYMENT_TARGET=11.0` and creates a Finder-launchable `Daozai.app` in the Cargo release output.
- Populate `Contents/Info.plist` with the executable name, app version, bundle identifier, and `LSMinimumSystemVersion`. Reuse the existing Daozai logo to produce the Finder icon.
- The first bundle is for local validation. Public signing, notarization, DMG distribution, and automatic updates remain outside this milestone.

### Documentation

- Update README system requirements and build/run instructions for Apple Silicon macOS 11+.
- Describe which first-release pages work and which system integrations remain hidden.
- Keep Windows packaging instructions and release artifacts unchanged.

## Data flow

```text
AppKit screen and pointer state
            ↓
macOS shell adapter → eframe viewport and `App::Env`
            ↓
shared App UI ← platform capability profile → visible pages and services
            ↓
Settings and local content → DAOZAI_DATA_DIR or ~/Library/Application Support/Daozai

macOS renderer → WGPU Metal → Glow fallback
```

## Compatibility and risks

- The current full binary fails macOS compilation because two `Env` fields read `system_state` outside Windows-only code. Use optional values for unsupported fields and preserve Windows behavior.
- `NSScreen.safeAreaInsets` may be unavailable at the macOS 11 deployment target. Check availability at runtime and retain a screen-frame fallback.
- macOS screen coordinates use a different origin from egui's top-left logical coordinates. Keep conversion inside the macOS adapter and validate Retina scaling and display placement on the host Mac.
- A transparent window can intercept input outside the island. Treat mouse pass-through and reactivation as an acceptance-critical behavior.
- The current host runs macOS 27.2.1, so runtime coverage is available only on that system. The app bundle and binary deployment target will state macOS 11, while minimum-version runtime testing remains unverified.
- The first `.app` is unsigned and intended for local validation, not public downloads.

## Rollback

Keep macOS-specific entry points target-gated. Revert the macOS capability profile or adapter independently. Existing serialized settings remain readable, including values for hidden Windows-only features.
