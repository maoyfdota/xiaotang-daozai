# macOS Bluetooth acceptance — 2026-11-25

Implementation and verification were performed inline by the main session, respecting the user's no-subagent instruction. Baseline: `my_feature_1123@4bba99e`.

## Automated results

Logs remain locally under `/tmp/daozai-mac-bluetooth-tests/`.

| Check | Result |
| --- | --- |
| Bluetooth filtered library tests | 24 passed, 1 ignored gallery (before final two narrowly scoped regression additions) |
| Final `bt::` library tests | 13 passed; includes blocked control deadline and cross-service native slot protection |
| Final main binary tests | 20 passed; includes modern/legacy settings destinations and existing capsule/scale regressions |
| Final settings all-tabs rendering | 1 passed, including Bluetooth tab 15 with modules enabled/disabled |
| Platform/page visibility/page order/settings mapping | 1 / 3 / 2 / 1 passed |
| Upgrade persistence regression | Exact previously failing test passed after expected tenth-page migration was updated |
| arm64 all-target check | Passed during implementation |
| Final product Clippy (`--lib --bin daozai --features record`) | Passed with existing warnings |
| Final all-target Clippy | Three existing `unused_io_amount` errors: `src/lyrics.rs:899`, `src/lyrics.rs:902`, `src/updates.rs:638`; verified present in HEAD |
| Full test comparison | Baseline 868 passed / 168 failed / 84 ignored; implementation run 882 passed / 169 failed / 85 ignored. The only added failure was the tenth-page expectation above, subsequently fixed and individually passed |
| Synthetic light/dark gallery | Passed; six local previews. Synthetic fixtures are not hardware evidence |
| Diff whitespace | Passed |

The 168 existing full-suite failures and their names are retained in `test-baseline.json`. This task does not claim a green repository-wide suite.

## Real-device results

Host: Apple Silicon, macOS 27.2.1 (26B131). User settings retained, including the right-docked 90% glass capsule. No real device names or addresses are recorded here.

- Bluetooth page reached through the fifth Mac page; permission-required UI was distinct from an empty device list.
- Once access was available, native enumeration displayed five paired devices, connected devices first. Scroll reached the final records without changing page.
- A connected headset displayed a valid 100% system-provided aggregate battery reading. Devices without supported battery data displayed unknown.
- Headset direct disconnect completed with `设备已断开`; the row changed to disconnected and battery became unknown.
- Direct reconnect showed progress, then `设备已连接` and connected state. The preconnection battery reading was not reused immediately after reconnect.
- The user initially reported no audio, then explicitly corrected the result: `不用验证了，我试过了，声音正常播放`. Record headset playback after reconnect as user-confirmed PASS. Stop further hardware interaction as requested.
- System Sound settings independently listed the headset as the selected output at 53%, unmuted. No output-device or volume setting was changed.
- Devices with class-zero endpoints used the approved system-settings fallback. No keyboard was disconnected. Direct keyboard input restoration remains untested; its confirmation/cancel behavior is covered with synthetic UI tests.
- Initial Bluetooth settings link reached General despite a successful open status. Fixed it to the actual installed settings extension identifier. The final installed build visibly opened the Bluetooth page; the privacy entry visibly opened the Bluetooth permission pane.
- After final unsigned bundle replacement the application again displayed its permission-required view, while System Settings showed its Bluetooth permission entry enabled. Final postreplacement enumeration was not revalidated before the user stopped hardware checks. No TCC reset or permission toggling was performed. Do not treat the final permission screen as a successful live enumeration.

## Acceptance mapping

| Criteria | Evidence / limit |
| --- | --- |
| AC-01 | Page entry and 90% capsule observed live; scale/gear/settings behavior covered by existing binary tests |
| AC-02 | Five live paired records and scrolling; stable identity/same-name behavior covered by fixtures |
| AC-03 | Real valid/unknown display; synthetic boundary and source-failure cases |
| AC-04 | Eight/nine/ten page migration and upgrade round-trip tests |
| AC-05 | Real headset disconnect/reconnect; synthetic duplicate, failure and timeout correlation |
| AC-06 | Synthetic keyboard cancellation/confirmation; no disruptive keyboard hardware operation |
| AC-07 | Permission UI plus live settings and privacy destinations; class-zero fallback observed |
| AC-08 | Independent service workers and request correlation tested; no claim of every hardware removal race being physically exercised |
| AC-09 | Management/reminder independence and initialization/failure baseline tests |
| AC-10–13 | Pure threshold/cycle tests plus UI queue, display acknowledgement, persistence, switch and hidden-page tests |
| AC-14 | Targeted checks and release passed; unrelated full-suite and Clippy test baselines disclosed |
| AC-15 | Headset state/control/battery and user-confirmed audio covered; final unsigned-replacement access state and direct keyboard input limits recorded above |

## Delivery

Updated the existing `/Applications/Daozai.app`, version 0.6.66, bundle ID `com.renly1994.daozai`. Installed/release binary SHA-256: `8fc7ef2e00662f7c5845fe430a85ec7ef236ce1ea28870429597bc90f524e95e`.

Verified arm64 Mach-O, valid Info.plist, Bluetooth usage description, `LSMinimumSystemVersion=11.0`, Mach-O `minos 11.0`, and running PID 74306 at `/Applications/Daozai.app/Contents/MacOS/daozai`. Prior bundles remain recoverable under `.git/local-backups/`; exact paths are in `installation.json`.

macOS 11 runtime, Intel, signing/notarization and real low-battery depletion were not tested. No push was performed. The user approved the code commit with `提交` on 2026-11-25. Keep the task in progress and do not archive while final installed permission access remains unresolved.
