# Mac 适配实施计划

## Ordered checklist

1. Re-run the Apple Silicon library and binary checks to confirm the recorded baseline. Inspect every macOS diagnostic before changing code.
2. Add the central platform capability profile. Filter first-release pages, page order, page settings, status controls, navigation, reminders, and optional background services consistently. Gate quota initialization/migration, quota service creation, media polling, QWeather credential access, `--calendar`, and direct hidden-page commands. Preserve saved hidden-feature values and the serialized settings schema. Add tests for stale all-enabled settings, hidden-page requests, and fallback navigation.
3. Change `model::config_dir()` to use `~/Library/Application Support/Daozai` on macOS while preserving `DAOZAI_DATA_DIR` and the Windows path. Add directory-selection and settings round-trip tests.
4. Add the macOS shell adapter for right-edge, vertically centered placement using the side canvas, visible-frame and notch-safe-area handling, global pointer sampling, mouse passthrough, and the existing expand/collapse flow. Resolve the current `system_state` cfg errors without changing Windows behavior. Implement native opening of the local data directory from visible settings.
5. Select Metal as the primary macOS renderer with Glow fallback. Keep the Windows OpenGL/DX12/Vulkan ordering unchanged.
6. Add `scripts/package-macos.sh` to build arm64 with deployment target 11.0 and assemble `target/aarch64-apple-darwin/release/Daozai.app`, including Info.plist and icon resources.
7. Update README with macOS 11+ Apple Silicon requirements, the first-release feature boundary, build instructions, and local app launch instructions.
8. Run automated tests and target checks. Open the generated `.app` on the current Mac and verify the island, settings, local persistence, renderer, display placement, menu bar/notch clearance, and mouse passthrough.
9. Inspect the app bundle metadata and linked deployment target. Review that Windows-only functionality is unavailable in the Mac bundle and confirm no unrelated worktree files were changed.

## Validation commands

```bash
rtk cargo check --locked --lib --target aarch64-apple-darwin
rtk proxy env MACOSX_DEPLOYMENT_TARGET=11.0 cargo check --locked --bin daozai --target aarch64-apple-darwin
rtk proxy env MACOSX_DEPLOYMENT_TARGET=11.0 cargo test --locked --lib --target aarch64-apple-darwin
rtk proxy env MACOSX_DEPLOYMENT_TARGET=11.0 bash scripts/package-macos.sh
rtk proxy plutil -lint target/aarch64-apple-darwin/release/Daozai.app/Contents/Info.plist
rtk proxy otool -l target/aarch64-apple-darwin/release/Daozai.app/Contents/MacOS/daozai
rtk git diff --check
```

The bundle launch check uses `open target/aarch64-apple-darwin/release/Daozai.app`. On this host, manual runtime checks cover macOS 27.2.1. Deployment-target validation does not count as a runtime test on macOS 11.

## Verification status

- `MACOSX_DEPLOYMENT_TARGET=11.0 bash scripts/package-macos.sh` completed and rebuilt `target/aarch64-apple-darwin/release/Daozai.app` after the latest source and documentation changes.
- `plutil -lint` passed for `Contents/Info.plist`. The bundle reports version `0.6.66`, bundle id `com.renly1994.daozai`, and minimum system version `11.0`.
- The earlier package-only reductions to 75% and 55% did not fix the running Dock icon. `main.rs` supplies `theme::app_icon()` to eframe, which calls `NSApplication::setApplicationIconImage` with that full-frame image after launch. The macOS runtime image now centers 80% artwork in its cached 128 × 128 transparent canvas. Packaging uses the same proportion. The rebuilt 1024 × 1024 ICNS has alpha bounds `(102, 111, 922, 912)`.
- The new `macos_app_icon_keeps_dock_padding` regression test failed against the original runtime image with bounds `(0, 1, 128, 127)`. After the fix, both icon tests passed. The release bundle rebuilt successfully, and LLDB read `NSApplication.applicationIconImage` directly from the restarted process. Its TIFF rasterized at 256 × 256 with alpha bounds `(25, 27, 231, 229)`, confirming 80.47% artwork width and transparent margins in the live app. `NSRunningApplication.icon` still returned the cached 86.52% workspace representation, so that API is insufficient to validate this runtime override.
- README and the location-settings hint now describe the right-side vertically centered Mac island. The program Dock artwork uses transparent padding to reduce its apparent size slightly.
- `file` identifies the executable as arm64 Mach-O. `otool -l` reports `LC_BUILD_VERSION` with `minos 11.0`.
- The old process was terminated and the rebuilt `.app` relaunched. PID 90212 runs from `target/aarch64-apple-darwin/release/Daozai.app/Contents/MacOS/daozai`. The bundle executable matches the release executable, with SHA-256 `84491629de56e164542e2ecc3dc82cfab521b58bf5ab3cc41bec42dbc09dd5c9`. CUA captured the visible right-side island with clock, weather, and reminder icons. Dock accessibility continued to time out, and the system screenshot application could not launch. A visual comparison of the Dock icon against neighboring apps remains pending despite the verified runtime image.
- The launch configuration lets the macOS root viewport start visible and active. Vendored eframe honors that visibility setting while retaining hidden-first-frame behavior on other targets.
- The 13 focused UI regression tests fixed by the test-alignment review passed. The final full macOS library run reported 867 passed, 168 failed, and 84 ignored. The full suite is not passing. Failures include expectations for modules deliberately hidden on macOS, platform/layout assumptions, and other tests that have not been classified. Do not mark all failures expected.
- Focused capability, data-path, reminder-settings, hidden-page entry, and settings-tab tests passed. The 15 existing `display_layout` tests passed. There is no direct `side_origin` test yet.
- `cargo check --locked --bin daozai --target aarch64-apple-darwin` with `MACOSX_DEPLOYMENT_TARGET=11.0` passed in the earlier implementation pass. After the runtime icon fix, `cargo clippy --locked --lib --target aarch64-apple-darwin` passed with 176 existing warnings and no diagnostics in `theme.rs`. The package script syntax check, bundle plist validation, and `git diff --check` passed.
- GUI checks for expansion/collapse, pointer passthrough, menu-bar/notch clearance, and persistence across restart remain pending. CUA now captures the visible island window. macOS 11 runtime behavior is also untested.

## Risk and review gates

- Check platform capability filtering together across navigation, settings, persisted page order, and service startup.
- Test startup with default settings, stale hidden-page flags, and `--calendar`. Confirm hidden services, credential discovery, reminders, and migration writes remain inactive and saved hidden-feature configuration remains unchanged.
- Check macOS file-opening behavior exposed by visible settings. Verify hidden Shelf actions cannot be reached through stale navigation.
- Check menu-bar and notch positioning in logical and physical coordinates on the current Retina display.
- Check transparent hit testing before accepting the Mac shell. Clicks outside the island must reach the window below it.
- Check existing settings and local content survive application restarts in the macOS data directory.
- This task's acceptance target is the Apple Silicon macOS bundle. Windows-native behavior and Windows runtime tests are outside this task's acceptance scope.
- Public signing, notarization, DMG generation, Intel builds, and macOS 11 runtime testing are explicitly deferred.
