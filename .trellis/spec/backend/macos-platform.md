# macOS Platform Contract

## 1. Scope / Trigger

This contract applies to the Apple Silicon macOS application target. It records the platform boundary introduced for the first macOS release, including local persistence, native display state, feature availability, rendering, and app packaging.

The first release targets `aarch64-apple-darwin` with macOS 11.0 as its deployment target. It does not claim Intel or macOS 11 runtime coverage.

## 2. Signatures

- `platform::page_available(page: usize) -> bool` owns the target-specific page capability profile.
- `model::config_dir() -> PathBuf` resolves the application data directory.
- `macos_shell::connected() -> Vec<display_layout::Display>` enumerates AppKit screens.
- `macos_shell::cursor_logical() -> Option<(f32, f32)>` samples the global pointer in logical desktop coordinates.
- `macos_shell::pointer_in_viewport(point, origin, scale) -> egui::Pos2` converts an AppKit desktop point to the current root frame's local egui coordinates.
- `macos_shell::logical_position(display_id: &str, physical: (i32, i32)) -> Option<egui::Pos2>` converts a physical display coordinate into the logical origin expected by the window viewport.
- `display_layout::side_origin(bounds, dock, canvas, half_extent, scale, offset) -> Option<(i32, i32)>` computes a bounded side-canvas origin in physical pixels.
- `scripts/package-macos.sh` builds the arm64 release executable and creates `target/aarch64-apple-darwin/release/Daozai.app`.

## 3. Contracts

### Capabilities

- macOS exposes focus, water/sit reminders, to-do, Open-Meteo weather, Bluetooth management (ID 9), and the system performance page (ID 10).
- macOS hides Music, Shelf, Mail, AI quota, Calendar, QWeather credentials, Windows notification delivery, Windows system status, startup, and foreground-window avoidance controls.
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
- Root input has an additional island-size scale. Capture the applied scale in `Shell::raw_input_hook`, before `update` can schedule another scale. Divide the AppKit pointer by that captured scale times `ctx.zoom_factor()`, then subtract the already-scaled viewport origin. An unavailable origin produces no pointer sample for that frame.
- Initialize the root scale before the first frame. Resize the native root canvas with `InnerSize(SIDE_SIZE * root_input_scale)` when placing it, so drawing and native bounds continue to agree after size changes. The independent settings viewport retains its own scale.
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
| Island size is changed to 90% or 125% | Capsule hover expands the panel, the gear opens settings, and leaving restores pointer passthrough. |
| The settings UI schedules a new island size during a frame | Pointer conversion uses the scale already applied to that frame's input. |
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
- `macos_shell::tests::scaled_capsule_hover_opens_panel_and_settings` feeds the shell's coordinate conversion into `App::frame` at 90%, 100%, and 125%, using positive and negative desktop origins. It asserts expansion, pointer capture, gear-click settings access, collapse, and restored passthrough. Unscaled subtraction must fail this regression at 90%.
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

## Bluetooth extension (2026-11-25)

- Mac Bluetooth management and its independent reminder switches use native capability and permission states, not the Windows notification-controls gate.
- Keep Windows legacy Bluetooth service consumers compatible. Append page ID 9 without changing existing page IDs or hidden persisted values.
- Use a packaged NSBluetoothAlwaysUsageDescription before requesting access. Denied permission is not an empty device list.
- Device operations run on bounded background workers; actual state confirms success and UI deadlines remain responsive. Keyboard disconnect requires confirmation.
- Unknown/failed battery is never zero; only new valid observations drive <=20% cycles. Persist dedup only after actual banner display.
- Do not merge device identity by name or infer authoritative removal from an incomplete source.

## Performance page extension (2026-11-26)

### 1. Scope / Trigger

The macOS performance page (page ID 10, settings tab ID 16) reports CPU and physical-memory use plus disk and network byte rates. It samples only while the performance page is open and enabled. Bluetooth remains page ID 9 and settings tab ID 15. Process lists, GPU metrics, temperature, and resident performance widgets are outside this contract.

### 2. Signatures

- `performance::collect() -> RawSample` reads one native macOS counter snapshot.
- `Sampler::due(&self, now: Instant) -> bool` gates sampling to about once per second.
- `Sampler::ingest(&mut self, now: Instant, raw: RawSample) -> Reading` computes one displayable reading and appends it to history.
- `Sampler::latest(&self) -> Reading` returns the latest reading; each metric is an `Option`.
- `Sampler::samples(&self) -> impl Iterator<Item = &Sample>` returns the retained trend points.
- `App::frame` calls `collect` only when `open`, `page == PERFORMANCE_PAGE`, and `module_on(PERFORMANCE_PAGE)` are all true.

`RawSample` contains optional CPU busy/total counters, memory used/total bytes, disk read/write cumulative bytes, and network receive/send cumulative bytes. `Reading` contains optional CPU percent, memory used/total bytes, and four per-second byte rates.

### 3. Contracts

- CPU uses Mach per-processor cumulative load counters. Read `natural_t` ticks as unsigned; interpreting the same 32-bit values as signed breaks after long uptime. Utilization is calculated only between two valid snapshots and a positive sampling interval; the first sample establishes the baseline.
- Memory uses `hw.memsize` and Mach free-page count × page size. Zero total, invalid page size, or an unavailable native query leaves memory unavailable.
- Disk uses cumulative `IOBlockStorageDriver` `Statistics` values (`Bytes (Read)` and `Bytes (Write)`). Network uses cumulative bytes on active non-loopback interfaces.
- Disk and network rates are counter deltas divided by the actual elapsed seconds. Each metric independently remains `None` if its source is missing, the interval is zero, or its counter moves backwards. Every accepted raw sample becomes the next baseline, including a missing or reset source.
- The history is capped at 60 points and 60 seconds. Missing values remain `None`; trend drawing leaves a gap across unavailable points.
- The UI labels unavailable values `不可用`; it must not format absent or invalid values as zero.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| First CPU/disk/network sample | Show unavailable while storing the counter baseline. |
| Missing native source or failed read | Keep only the affected metrics unavailable; do not substitute zero. |
| Zero or invalid interval | Do not calculate CPU utilization or byte rates. |
| Counter decreases | Leave that metric unavailable and use the new sample as the next baseline. |
| Memory total is zero or used exceeds total | Leave memory unavailable. |
| Performance page closes, is disabled, or another page is selected | Stop sampling and stop its one-second repaint request. |
| A trend point lacks a metric | Preserve a gap for that series. |

### 5. Good / Base / Bad Cases

- **Good**: Two valid snapshots two seconds apart show the accumulated byte delta divided by two, while each trend series has its own valid points.
- **Base**: On opening the page, CPU and cumulative byte rates are unavailable until a second sample; memory can display as soon as its absolute counters are valid.
- **Bad**: A missing disk counter is rendered as `0 B/s`, or a line bridges a missing sample as if it were measured. Both hide unavailable data and are forbidden.

### 6. Tests Required

- `cumulative_counter_deltas_use_the_actual_sampling_interval`: assert CPU percentage and read/write/network rates from known deltas and elapsed seconds.
- `first_samples_missing_sources_and_counter_resets_remain_unavailable`: assert no fabricated values and baseline recovery after missing/reset counters.
- `invalid_cpu_deltas_and_memory_values_are_not_fabricated`: assert zero CPU total delta and invalid memory totals remain unavailable.
- `cpu_and_rate_values_need_a_positive_sampling_interval`: assert same-instant samples produce no CPU percentage or byte rate.
- `trend_keeps_unavailable_gaps_and_only_retains_about_one_minute`: assert gaps are kept and history remains bounded.
- `performance_page_configuration_migrates_old_lengths_and_round_trips_eleven_entries`, `macos_first_release_exposes_only_local_core_pages`, and settings-tab mapping tests assert page IDs, platform gating, and config compatibility.
- `cargo check --release --target aarch64-apple-darwin --lib` validates the Apple Silicon native API bindings.

### 7. Wrong vs Correct

#### Wrong

```rust
let rate = current.unwrap_or_default().saturating_sub(previous) as f64 / seconds;
```

This turns a missing counter or reset into a plausible zero rate.

#### Correct

```rust
let rate = current.checked_sub(previous)? as f64 / seconds;
```

Propagate unavailable data through `Option` so the card and trend can show a gap.
