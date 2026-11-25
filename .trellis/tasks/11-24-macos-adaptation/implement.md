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
- The packaging script scales every iconset image to 75% within a transparent square canvas before creating the `.icns`. `iconutil` successfully extracted the rebuilt 1024 × 1024 icon, whose corner pixels are transparent and whose artwork bounds are smaller than the canvas.
- README and the location-settings hint now describe the right-side vertically centered Mac island. The program Dock artwork uses transparent padding to reduce its apparent size slightly.
- `file` identifies the executable as arm64 Mach-O. `otool -l` reports `LC_BUILD_VERSION` with `minos 11.0`.
- The process running during the bundle rebuild had started beforehand, so it was terminated and relaunched from the rebuilt `.app`. The new process runs from `target/aarch64-apple-darwin/release/Daozai.app/Contents/MacOS/daozai`. The desktop was locked during the follow-up capture, so the Dock icon could not be visually compared with neighboring icons. Visible rendering and right-side placement remain unverified.
- The launch configuration lets the macOS root viewport start visible and active. Vendored eframe honors that visibility setting while retaining hidden-first-frame behavior on other targets.
- The 13 focused UI regression tests fixed by the test-alignment review passed. The final full macOS library run reported 867 passed, 168 failed, and 84 ignored. The full suite is not passing. Failures include expectations for modules deliberately hidden on macOS, platform/layout assumptions, and other tests that have not been classified. Do not mark all failures expected.
- Focused capability, data-path, reminder-settings, hidden-page entry, and settings-tab tests passed. The 15 existing `display_layout` tests passed. There is no direct `side_origin` test yet.
- `cargo check --locked --bin daozai --target aarch64-apple-darwin` with `MACOSX_DEPLOYMENT_TARGET=11.0` passed. `cargo clippy --locked --lib --target aarch64-apple-darwin` passed with 212 warnings. `git diff --check` passed before the final spec and task-record edits and must be rerun.
- GUI checks for expansion/collapse, pointer passthrough, menu-bar/notch clearance, and persistence across restart remain pending because the app window was unavailable through CUA. macOS 11 runtime behavior is also untested.

## Risk and review gates

- Check platform capability filtering together across navigation, settings, persisted page order, and service startup.
- Test startup with default settings, stale hidden-page flags, and `--calendar`. Confirm hidden services, credential discovery, reminders, and migration writes remain inactive and saved hidden-feature configuration remains unchanged.
- Check macOS file-opening behavior exposed by visible settings. Verify hidden Shelf actions cannot be reached through stale navigation.
- Check menu-bar and notch positioning in logical and physical coordinates on the current Retina display.
- Check transparent hit testing before accepting the Mac shell. Clicks outside the island must reach the window below it.
- Check existing settings and local content survive application restarts in the macOS data directory.
- This task's acceptance target is the Apple Silicon macOS bundle. Windows-native behavior and Windows runtime tests are outside this task's acceptance scope.
- Public signing, notarization, DMG generation, Intel builds, and macOS 11 runtime testing are explicitly deferred.
