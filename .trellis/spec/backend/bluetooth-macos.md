# Mac Bluetooth Contracts

## 1. Scope / Trigger

Applies to the Mac-only management page and native adapter. The legacy Windows connected-device service remains compatible. This feature changes native/service/UI commands, page configuration and a versioned sidecar file.

## 2. Signatures

- `bt::management::spawn() -> Service`; `Service::send(Command) -> Result<(), String>`; `Service::rx: Receiver<Event>`.
- `App::receive_bluetooth_event(Event)` consumes ordered snapshots and operation results; `App::reset_bluetooth()` resets a service generation.
- `Env::managed_bt: Option<Snapshot>` is the Mac path; `Env::bt` and `bt_ready` retain legacy semantics.
- `Cmd::{BluetoothRefresh, BluetoothControl(Request), RequestBluetoothAccess, OpenBluetoothSettings, OpenBluetoothPrivacy}` own shell effects.
- `BluetoothAlertStore` writes version 1 `bluetooth-alerts.json` with `shown` device IDs through `model::atomic_write`.

## 3. Contracts

Device identity uses normalized 48-bit addresses; names only display/sort. Device state distinguishes Connected/Disconnected/Unknown. BatteryStatus is separate from optional value; valid values are 0–100 inclusive. Repeated UI frames retain observation sequence.

Mac reads paired native devices via IOBluetooth and optional aggregate battery via a bounded system-profiler query. Its diagnostic schema is not a stable API. Denied/restricted access cannot be bypassed through the diagnostic source. A pre-reconnection report cannot supply a post-reconnection battery observation.

A control request carries runtime ID, device ID and Action. Only the current matching result releases UI pending state. Native return success is not connection success: confirm new system state. UI timeout is 20 seconds; native worker capacity stays occupied until the call actually returns. No duplicate worker is created for retry. Keyboard disconnection requires explicit confirmation.

The Mac native control slot is process-wide. Dropping/recreating a service does not allow a second native call while the previous generation remains blocked; use a nonblocking lock and return a retry explanation.

The tenth persisted page has ID 9 and settings ID 15. Mac available pages are [1,2,4,8,9]. Eight/nine-page permutations retain order and append missing pages; hidden legacy values survive saves. The page is Mac-only even though shared configuration stores ten slots.

Both reminder switches are independent of management. New valid connected battery <=20% is eligible once per cycle, >20% resets. Reconnect/rename/restart preserve shown IDs. Actual visible drawing acknowledges display; queue insertion does not. Unknown/failed/old data cannot reset or trigger a cycle.

`NSBluetoothAlwaysUsageDescription` must be present before explicitly requesting access through the packaged app. Default data remains ~/Library/Application Support/Daozai; DAOZAI_DATA_DIR isolates tests.

Bluetooth settings target the installed `Bluetooth.appex` extension identifier `com.apple.BluetoothSettings` when present, otherwise the legacy `com.apple.preferences.Bluetooth` pane. `com.apple.Bluetooth` can return a successful open status while displaying the wrong settings page. Verify the visible destination, and provide location instructions when falling back to the settings application.

## 4. Validation and Error Matrix

| Condition | Behavior |
| --- | --- |
| Authorization not decided/denied/restricted | PermissionRequired with permission/settings actions; no fabricated empty list. |
| Bluetooth off or controller missing | Unavailable; known information marked unknown, no false disconnect storm. |
| Battery missing/invalid/report fails | Unknown/Failed, never zero; skip low-battery decisions. |
| Control returns but state does not change | Continue pending until a fresh target state or timeout. |
| Native call remains blocked after UI timeout | Keep the single native slot occupied and reject duplicate work. |
| Late result for previous request | Do not clear the new request. |
| Dedup write fails | Keep memory dedup, retry after 30 seconds, record bounded diagnostics. |
| All native/diagnostic information unavailable | Explain limitation and provide Mac settings fallback. |

## 5. Good / Base / Bad Cases

- Good: A connected keyboard at 20% produces one actually displayed reminder; restarting still deduplicates until a valid >20% reading.
- Base: No paired devices produces an empty-state UI when the native source succeeds.
- Bad: Two devices share a name. Keep distinct address identities and requests.
- Bad: User cancels keyboard confirmation. Emit no disconnect command.

## 6. Required Tests

- `bt::management::tests` exercises permission guards and a blocked native call whose UI timeout does not spawn duplicate native work.
- `bt::macos::tests` accepts aggregate battery at 0 and 100, rejects invalid values, offline data and name-only identity.
- Native control tests reject a later service generation while the process-wide control slot is held; binary tests cover modern/legacy settings targets.
- `bt::low_battery::tests` covers threshold/recovery, old/failed observations, re-enable baseline and persisted shown IDs.
- `ui_tests::bluetooth_macos` covers confirmation, operation correlation, permission actions, independent switches, hidden page, long-list scroll, configuration migration and actual-display persistence.
- Existing binary capsule/scale tests must pass; synthetic light/dark gallery is separate from real hardware evidence.

## 7. Wrong vs Correct

```rust
// Wrong: native call accepted does not establish target state.
if native_return == 0 { complete(request); }

// Correct: match request identity and observe the target state.
if pending.request.id == result.request.id && target_reached(&snapshot, &pending.request) {
    complete(pending.request);
}
```

Hardware capabilities and system-cache freshness are recorded limits, not guarantees inferred from unit tests.

## Battery observation extension (2026-11-26)

- Mac battery readings carry a SystemReport source and successful query completion time; this is not a device measurement timestamp.
- Values expire after 90 seconds, including a snapshot retained by the UI without new service events. Failed queries may retain the previous value/time for history but cannot drive alerts.
- A manual refresh also requests a battery report, subject to a five-second minimum request interval and one in-flight report. Background refresh remains 60 seconds.
- Unknown native authorization values are unavailable states, not proof that the user denied permission. The installed-bundle permission discrepancy remains a hardware validation item.

## Wireless UI and residents (2026-11-26)

- The displayed page name is Wireless Devices; persisted page ID 9 and settings tab 15 are unchanged. The detail view retains native controls, keyboard confirmation, permission recovery and history/fresh-value separation.
- Wireless residents default off. Auto mode picks at most two connected devices, preferring valid battery readings ordered by level then stable identity; it fills unused slots with connected devices whose battery is unknown. Unknown/stale/failed battery remains an unknown placeholder and never becomes a fabricated level. Custom mode keeps at most two saved IDs across disconnect/removal and displays only currently connected selections; missing battery uses an unknown placeholder.
- Resident slot 10 is appended to previous seven-slot orders; six-slot orders first receive the existing weather migration. Side placement honors custom order. Settings previews must filter unavailable platform slots, including after restoring defaults.
- Ring-only and ring-with-number styles share the same source data and route to device details. Page visibility, reminder preferences and resident visibility remain separate.
- The Mac wireless page omits charging status, animation and alert settings. A future charging source must supply explicit state for the same normalized device identity; the host power state, a matching name, rising battery level and 100% are not evidence of peripheral charging.
