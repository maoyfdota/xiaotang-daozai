# macOS Bluetooth Implementation Plan

## Execution and approval

The main session performs research, implementation and checks directly. Do not use sub-agents. Read the context manifests inline; do not change global dispatch configuration.

Planning is complete only after the artifacts pass review. Present the latest final summary and obtain a subsequent explicit implementation approval before task.py start. All shell commands use rtk. Use CodeGraph before source lookup; adapt selected pure logic from the other branch without merging it.

## Ordered checkpoints

### 1. Baseline and contracts

- [x] Verify branch, working changes, task artifacts and applicable specs; record current Mac check/test failures as a baseline.
- [x] Inspect all tenth-page, configuration, constructor, atomic persistence and worker consumers.
- [x] Update the approved Mac platform contract before code changes and keep the change boundary in design.md current.

### 2. Native observations and managed state

- [x] Add domain contracts, permission states and Mac paired-device/state backend; preserve old Windows consumers.
- [x] Add tolerant system-report parsing with identity-safe merging and optional battery, source completeness and observation sequences.
- [x] Bound workers, subprocess time/output and event delivery; reap timed-out children.
- [x] Test empty versus unavailable, denied access, partial failure, same names, old service generations, refresh coalescing and shutdown.

### 3. Page, settings and migration

- [x] Add Mac-only page and settings, migrate ten-page consumers and old configurations without losing values.
- [x] Add sorted list, long names, scrolling, loading/empty/error/permission feedback, refresh and dedicated settings commands.
- [x] Verify five available Mac pages, 90%/100%/125% right-docked interaction and separate settings; inspect synthetic light/dark screenshots.

### 4. Device operations

- [x] Add supported native connect/disconnect with unique request IDs, keyboard confirmation and 20-second UI deadline.
- [x] Confirm fresh real state; keep operation execution separate from UI/deadline/state observation.
- [x] Test duplicate input, blocked native calls, retries, page switching, removal, external changes, late results and service recreation.
- [x] Record actual device audio/input behavior or the accepted capability fallback.

### 5. Independent notifications

- [x] Add initial connection baseline and independent switches without disabling management.
- [x] Adapt <=20% cycle logic, actual display acknowledgement and versioned atomic persistence.
- [x] Test 0/19/20/21/100, first low reading, recovery, rename, reconnect, restart, multiple same-name devices and four switch combinations.
- [x] Test stale/failed/denied input, pre-display invalidation, queue eviction, total reminder off, hidden page, corrupt storage and write failures.

### 6. Main-session check and delivery

- [x] Read trellis-check and review the complete diff against AC-01–AC-15 directly; fix new failures and document baseline failures.
- [x] Run appropriate checks below; avoid unrelated whole-repository formatting and repeated checks without new evidence.
- [x] Package and check permissions/settings/live list plus existing capsule and settings interaction.
- [x] Inspect existing Mac installation entry, update its app, verify hashes/bundle metadata/process path and keep a recoverable prior bundle.
- [ ] Commit after the one-shot plan confirmation, then record journal/archive according to the workflow. Specs and acceptance results are updated. Any authorized push must explicitly target verified origin and branch, never upstream.

## Validation

Set an isolated DAOZAI_DATA_DIR for tests. Use actual new test-module names when known.

```bash
rtk cargo check --locked --all-targets --features record --target aarch64-apple-darwin
rtk cargo test --locked --lib bt:: -- --test-threads=1
rtk cargo test --locked --lib platform::tests -- --test-threads=1
rtk cargo test --locked --lib model::tests -- --test-threads=1
rtk cargo test --locked --lib ui_tests::bluetooth -- --test-threads=1
rtk cargo test --locked --bin daozai -- --test-threads=1
rtk cargo clippy --locked --all-targets --features record --target aarch64-apple-darwin
rtk cargo test --locked --all-targets --features record -- --test-threads=1
rtk git diff --check
rtk proxy bash scripts/package-macos.sh
```

Explicitly run only relevant ignored synthetic screenshot cases. No real user addresses/names in committed screenshots. If repository-wide formatting already fails, record the baseline and format the changed scope. Verify plist with plutil, arm64/minos with Mach-O tooling, and SHA-256 against the installed executable. Separate direct hardware pass, fallback pass and pending checks.

## Review / rollback gates

| Gate | Evidence |
| --- | --- |
| Before implementation | Complete PRD/design/plan, no unresolved product decisions, subsequent final-summary approval. |
| Native read path | Failures distinct from empty data, stable identity, finite resources, honest source completeness. |
| Control | Request correlation, real state, keyboard confirmation, timeout feedback despite blocked calls. |
| Reminders | Management independence, actual display acknowledgement, restart dedup and valid recovery. |
| Delivery | New regressions resolved, baseline failures listed, installed bundle/process match, hardware limits recorded. |

No destructive reset/clean for rollback. Preserve unrelated edits; restore only this task's patch or prior Mac bundle without clearing user data.

## Final evidence / remaining limits

See `research/acceptance.md` for actual test counts, baseline failures, headset control and user-confirmed playback. Final bundle access after unsigned replacement remains explicitly unverified after the user stopped hardware checks; do not claim every hardware case passed. Direct keyboard operation uses the accepted fallback and was not physically exercised. The user approved the commit plan with `提交` on 2026-11-25. Record the code commit and session journal; keep the task in progress because final installed permission access remains unresolved.
