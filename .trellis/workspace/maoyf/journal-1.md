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


## Session 2: macOS 蓝牙管理
<!-- trellis-session: v=2 fp=8926ac5d4612f7f3 -->

**Date**: 2026-11-25
**Task**: macOS 蓝牙管理
**Branch**: `my_feature_1123`

### Summary

完成 Mac 蓝牙设备管理、连接控制、独立连接及低电量提醒，已按用户确认清单提交并交付到 /Applications/Daozai.app。用户确认耳机重连后声音正常，要求停止进一步实机验证。

### Main Changes

- 新增 Mac 蓝牙页面和设置、IOBluetooth 已配对设备管理、有界后台控制与电量查询、跨重启低电量提醒去重；更新平台契约和验收记录。
- 保留任务 in_progress：最终未签名应用替换后，系统权限开关与应用权限提示存在不一致，最终设备枚举尚未复验；没有推送。

### Git Commits

| Hash | Message |
|------|---------|
| `17bbdfbf09118b6d9c241a7f8c9c761f86c0e40d` | feat(macos): add Bluetooth device management and reminders |

### Testing

- [OK] 已完成的最终检查：bt:: 13 项通过、主程序 20 项通过、设置页渲染通过、arm64 all-target check 通过、产品 Clippy 通过；本次仅提交，暂存 diff --check 通过。
- [OK] 全库基线有 168 项既有失败；实现全量运行额外出现的页面迁移断言已修正并定向通过。all-target Clippy 保留 3 项既有 unused_io_amount 错误。
- [OK] 实机耳机连接、断开、重连和电量显示已覆盖；重连后音频播放由用户确认正常。最终应用替换后权限状态仍为未验证。

### Status

[OK] **Completed**

### Next Steps

- 保留已记录的最终安装权限待确认事项，等待用户后续安排；本次不继续实机操作，不归档为完全验收完成。
