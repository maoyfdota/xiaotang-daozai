# Journal - maoyf (Part 1)

> AI development session journal
> Started: 2026-11-19

---



## Session 1: Repair scaled macOS capsule settings access
<!-- trellis-session: v=2 fp=a206be8ce7b6bec5 -->

**Date**: 2026-11-25
**Task**: Repair scaled macOS capsule settings access
**Branch**: `my_feature_1123`

### Summary

Fixed global pointer conversion for the 90% macOS capsule. User confirmed hover expansion and settings access. Installed the arm64 macOS 11 release into /Applications/Daozai.app.

### Main Changes

- Capture the applied root scale before settings changes and align AppKit pointer coordinates and native canvas bounds.

### Git Commits

| Hash | Message |
|------|---------|
| `e92cc38` | fix(macos): align capsule hover with island scale |

### Testing

- [OK] 19 binary tests, 7 model tests, and 4 settings-window tests passed. Clippy and release bundle checks passed with existing warnings.
- [OK] User verified physical-pointer hover and gear access. CUA synthetic events do not update the global AppKit mouse position on this host.

### Status

[OK] **Completed**

### Next Steps

- Continue outstanding macOS adaptation acceptance checks. The earlier full library suite still has 168 unclassified failures.
