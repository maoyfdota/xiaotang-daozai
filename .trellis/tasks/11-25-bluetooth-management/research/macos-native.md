# macOS Native Bluetooth Evidence

Checked 2026-11-25 directly by the main session. No connect/disconnect, pairing or permission reset was performed.

## Primary SDK evidence

SDK: /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk.

IOBluetooth.framework/Headers/objc/IOBluetoothDevice.h:

- 560–565: isConnected reports baseband connectivity, not audio routing or usable HID input.
- 568–582: synchronous openConnection waits for success/failure/timeout; already-connected errors need state-aware handling.
- 585–622: callback overload may return success when CREATE_CONNECTION was merely issued; page-timeout overload is available.
- 625–632: closeConnection waits synchronously for closure or failure.
- 872–891: pairedDevices returns paired objects (nil when none), isPaired reports pairing. These declarations alone do not prove full BLE coverage.

CoreBluetooth.framework/Headers/CBManager.h, 51–86: not-determined/restricted/denied/allowed authorization; class authorization is available since macOS 10.15, compatible with deployment target 11.

CoreBluetooth.framework/Headers/CBCentralManager.h, 144–165: retrieve by known UUIDs or retrieve connected peripherals implementing requested services. Neither is an all-paired-BLE enumeration contract.

## Apple reference locations

- https://developer.apple.com/documentation/iobluetooth
- https://developer.apple.com/documentation/iobluetooth/iobluetoothdevice/paireddevices()
- https://developer.apple.com/documentation/iobluetooth/iobluetoothdevice/openconnection()
- https://developer.apple.com/documentation/iobluetooth/iobluetoothdevice/openconnection(_:)
- https://developer.apple.com/documentation/iobluetooth/iobluetoothdevice/closeconnection()
- https://developer.apple.com/documentation/corebluetooth/cbmanager/authorization
- https://developer.apple.com/documentation/bundleresources/information-property-list/nsbluetoothalwaysusagedescription

Web fetch/search returned no usable content in this session. API behavior above is verified from the installed Apple SDK headers; the URL list is for follow-up reference, not a claim of successfully fetched pages. Package the usage string before permission requests, and verify attribution by launching the app bundle rather than a test binary under Terminal.

## Read-only host observation

/usr/sbin/system_profiler SPBluetoothDataType -json -detailLevel basic completed successfully. It returned SPBluetoothDataType, controller_properties and device_not_connected, containing four disconnected records; no connected section was present.

Observed device field names: device_address, device_firmwareVersion, device_minorType, device_productID, device_vendorID. No raw addresses/names were persisted. No battery or control capability was established by this observation.

Diagnostic JSON is not a stable application schema. Parsing must be optional and tolerant with explicit error/completeness, identity checks, time/output limits and process reaping. Missing data means unknown. A successful query of a system cache is not proof of fresh physical telemetry.

## Recommendation and bounded unknowns

Reuse objc 0.2.7 already in Cargo.toml; isolate Foundation/IOBluetooth/CoreBluetooth linking and reference ownership. Keep finite worker slots and UI deadlines independent of blocked native operations. Use supported native endpoints for classic control, and system-settings fallback for unverified BLE/profile behavior. Do not introduce a required third-party CLI/private API.

Battery and actual audio/HID success require hardware. Preserve macOS 11 deployment availability and make newer optional capabilities conditional. The accepted unknown-battery/unsupported-action behavior bounds these unknowns without claiming unsupported guarantees.
