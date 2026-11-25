# macOS Bluetooth Design

## Boundary and execution

Baseline: `my_feature_1123@4bba99e`. Mac currently has neither Bluetooth observations nor a management page. The change spans native access, background workers, domain state, UI and configuration.

The main session performs all work directly, per user instruction. Do not dispatch agents or change project-wide dispatch settings. Do not run `task.py start` or edit product code before the final planning review is approved.

Treat this as one integrated feature with sequential checkpoints, since listing, control and reminders share identity, events and migration. Adapt useful pure logic and layout from `my_feature_1119`; do not merge that branch wholesale.

## Modules and change boundary

| Files | Responsibility |
| --- | --- |
| New `src/bt/management.rs` | Bounded observation/control/deadline service; shared device/snapshot/request types remain in src/bt.rs. Reuse existing `bt::Device` identity/name/kind helpers while keeping old Windows service consumers compatible. |
| New `src/bt/macos.rs` | All Mac native calls, permission checks, source merging, endpoint mapping and object ownership. Compile only on Mac. |
| New `src/bt/low_battery.rs` | Adapt the other branch's pure low-battery lifecycle. No threshold logic duplicated in drawing or native code. |
| `src/app.rs`, `src/main.rs` | Ordered event consumption, pending requests, shell side effects and Mac service lifetime. Adapt Env only where required. |
| New `src/bluetooth_ui.rs`, `src/extras.rs`, `src/lib.rs` | Device page, progress, keyboard confirmation, settings, reminder lifecycle and module registration. |
| `src/platform.rs`, `src/model.rs` | Mac capabilities, tenth-page migration, independent switches and atomic dedup persistence. |
| `scripts/package-macos.sh` | Packaged Bluetooth usage description. |
| Related tests, recorder/examples | Update changed constructors and page-size consumers; add meaningful state, lifecycle and UI coverage. |
| `.trellis/spec/backend/macos-platform.md` | Record the approved Bluetooth extension while preserving the existing display, data and packaging contracts. |

Existing feature IDs stay stable. No Windows Bluetooth feature development or unrelated rendering/formatting changes.

## Domain and event contracts

- Managed device: stable ID, display name/kind, Connected/Disconnected/Unknown state, optional valid battery observation, and independent connect/disconnect capability with reasons.
- Identity: normalized address or authoritative system ID. Names are display/sort data. Do not merge records by equal names; keep source and controllable endpoint identities explicit.
- Snapshot: monotonic sequence, observation time, device list, Loading/Ready/Partial/PermissionRequired/Unavailable status and error reason. Only a known complete enumeration can confirm removal. A failed source retains known records with stale/unknown state.
- Battery: value 0–100, source/read status and observation sequence. Repeated frames of one observation cannot reset or retrigger a cycle. Failed, disconnected or timed-out sources invalidate decisions.
- Control: unique runtime request ID, device ID, action and deadline. Only the matching terminal result clears its pending operation; late results may update real state but cannot finish another request.
- Notifications consume the same managed state. Their two switches do not own the management service.

## Mac discovery, control and battery

Use public IOBluetooth pairedDevices, address/name/class and isConnected as the primary native source. SDK evidence is in `research/macos-native.md`.

Optionally supplement system-visible devices and battery via bounded `/usr/sbin/system_profiler SPBluetoothDataType -json`. This diagnostic schema is not a stable application API: use an isolated tolerant parser, explicit source status, output limits and fixtures. Missing fields mean unknown. Never add unpaired broadcast discoveries to the page.

CoreBluetooth retrieval requires known identifiers or service filters; it does not establish a complete all-paired-BLE list. A system-visible device without a verified control endpoint can still be shown with a settings fallback. Uncertain source completeness cannot imply unpairing.

Classic IOBluetooth openConnection/closeConnection are control candidates. Expose direct actions only with the corresponding valid endpoint and actual backend support. Confirm a new system state after a request. The native contract is baseband connectivity; actual audio/HID usability is a separate hardware check. Do not auto-switch audio routes.

UI operation deadline: 20 seconds. Execute native operations away from the UI and independently of state/deadline updates. A blocked native call must not prevent timeout feedback or normal interaction. Use fixed worker capacity and serialize outstanding native calls; after a UI timeout do not accumulate new workers or duplicate calls. Report/reject a retry that cannot safely run until the previous native call returns. Shutdown stops future work and ignores old-generation results.

Objective-C references remain owned by the worker using them, with autorelease pools and explicit retain/release. Transport Rust values through channels, not unowned native pointers.

Battery collection uses system-provided aggregate battery without connecting a device just to read it. Accept only 0–100 and reliably match identity; no name-only matching. Normal battery interval is 60 seconds; device changes/manual refresh can request a throttled update. One subprocess at a time, a 15-second limit, bounded output, and kill/wait on timeout. Missing/failed data becomes unknown; do not add private APIs or a required third-party CLI.

A successful system query is a new system observation, not proof of new physical telemetry. Record provenance/time; do not infer unseen charging recovery. Unknown battery means no low-battery notification.

## Permission and settings

Include NSBluetoothAlwaysUsageDescription in the packaged app. Use CBManager.authorization (available before macOS 11) to distinguish not-determined, restricted, denied and allowed states. Initialize permission-requesting objects only through the feature's user activation/authorization path when a decision is needed. Never reset TCC or repeatedly prompt on startup.

Denied/restricted access must not be bypassed with a fallback data source. After authorization the service can run for either the page or enabled reminders; if all three consumers are disabled, stop optional sampling.

Use dedicated shell commands for fixed Bluetooth/privacy settings targets, since the existing web URL command filters non-web schemes. Attempt the appropriate Mac settings destination and fall back to opening System Settings/System Preferences with an actionable location message. Verify on the host; no privileged settings editing.

## Page and configuration compatibility

Append Bluetooth page ID 9, using an unused settings ID (prefer the other branch's 15). Keep IDs 0–8 and existing settings mappings unchanged. Expand page storage/consumers to ten entries.

Mac's available set becomes [1,2,4,8,9]. Do not expose other hidden Mac features or enable the new management page on Windows. Preserve the old Windows notification source/service contract.

Valid eight/nine-page order retains its permutation and appends missing pages according to existing migration rules, then Bluetooth. Valid ten-page order round-trips. Preserve old visibility bits and all-hidden repair behavior; platform selection still picks an available Mac page. Bluetooth defaults visible at the end for ordinary existing configurations.

Preserve the existing bluetooth_notifications value; new bluetooth_low_battery_notifications defaults enabled. Put both editable switches in Mac Bluetooth settings without exposing Windows notification controls. Page visibility does not disable reminders.

Use the existing Card/icons and confirmation patterns, cancel as the safe default for keyboard disconnect. List scrolling must not navigate to another page. Preserve right docking, scaled capsule hover, and the independent settings viewport.

## Reminder lifecycle and storage

Connection reminders establish an initial baseline; failed/unknown observations do not become disconnect events. Re-enabling this reminder establishes a fresh baseline.

Maintain per-device not-shown/pending/shown low-battery state. Only a new valid connected observation <=20% can enqueue; >20% resets. Reconnect and rename preserve the cycle. Confirm shown only after the banner is actually visible, not when queued. Revalidate identity, cycle, switches and battery before display. Eviction/cancellation cannot consume the cycle.

Persist shown device IDs in versioned bluetooth-alerts.json under the existing data directory, using model::atomic_write. Read on startup; update on actual display, valid recovery and authoritative unpairing. Incomplete sources must not delete dedup records. Corrupt/missing data recovers safely with diagnostics. Write failure preserves in-memory dedup and uses bounded retry; an abnormal exit before persistence may cause one duplicate after restart and must be recorded honestly.

## Verification, delivery and rollback

Use isolated DAOZAI_DATA_DIR and synthetic fixtures for automatic tests. Record the current Mac failure baseline before changing code. Check native-source errors, identity collisions, request races, page migration, low-battery recovery and display confirmation.

Use the packaged app to check permission, settings, enumeration and real devices. Record direct-control pass, fallback pass or pending hardware validation separately; simulated data is never a hardware pass.

Inspect the existing Mac installation entry, expected /Applications/Daozai.app, before delivery. Build arm64, verify Info.plist/usage string/minos 11.0 and compare executable hashes and running path. The Windows shortcut instruction does not select a Mac install destination. Keep a recoverable previous app bundle. Roll back the feature patch/app bundle without clearing user data; old versions can ignore the additional setting and sidecar file.
