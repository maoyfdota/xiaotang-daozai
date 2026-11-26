# 蓝牙改造验证记录

日期：2026-11-26。目标：macOS / Apple Silicon。当前任务不要求 Windows 兼容。

## 软件验证

- `cargo test --lib bt:: -- --test-threads=1`：15 项通过，包含时效、失败历史、重连与授权映射。
- `cargo test --lib bluetooth -- --test-threads=1`：总览、详情、操作、提醒、配置、常驻集成定向回归通过；图库默认忽略，另行执行。
- `cargo test --lib wireless -- --test-threads=1`：筛选、配置、侧边点击、设置预览及恢复默认回归。
- `cargo test --lib bar_layout -- --test-threads=1`：10 项通过，含新无线设备槽位及原布局。
- `cargo check --target aarch64-apple-darwin --bin daozai`：通过，保留已有警告。
- `cargo test --bin daozai macos_shell::tests -- --test-threads=1`：通过，含 90%／100%／125% 缩放、展开、设置入口及点击穿透。
- `DAOZAI_BLUETOOTH_PREVIEW=/tmp/daozai-wireless-preview cargo test --lib bluetooth_macos_gallery -- --ignored --test-threads=1`：通过；检查深浅色总览、详情、权限、低电量提醒及常驻两种样式。截图是模拟数据，不作为实机连接和电量证据。

## 扩大回归的已有失败

`cargo test --lib resident -- --test-threads=1` 在本次代码上出现 10 个失败。用 `git archive 9019ca8` 创建独立临时源码副本，在同一 Mac 上运行相同命令，相同 10 个用例也失败。没有修改主工作区来做基线对照。

- clock_seconds_keep_resident_spacing_and_digit_width
- deepseek_resident_preview_does_not_overlap_icons
- island_lyrics_pause_returns_to_resident_and_resume_follows_current_position
- island_position_drag_clamps_to_existing_limits_and_stays_resident
- kiro_credits_follow_the_ring_on_the_resident_bar
- resident_components_can_be_dragged_saved_and_reset
- resident_fifth_account_stays_clear_of_clock_and_opens
- resident_quota_moves_left_and_releases_battery_space
- resident_quotas_coexist_with_music_and_focus
- weather_source::qweather_apply_switches_only_after_validation_and_module_switch_controls_source_and_resident

这些用例包含 Mac 未开放模块或顶部布局假设，不能据此宣布整个仓库测试通过。本次对无线设备另有 Mac 侧边布局、配置和交互定向回归；上述失败留作既有测试基线，不顺带适配排除模块。

## 权限与充电能力边界

已安装应用的 Info.plist 只读核对：包标识 com.renly1994.daozai，蓝牙用途说明存在。没有重置 TCC、替换应用、连接或断开实体设备。旧权限差异没有新实机复现，仍需后续确认。

本机 SDK 主来源：

- `/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreBluetooth.framework/Headers/CBManager.h`：授权枚举和类方法；未知值不能直接按拒绝处理。
- `/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Headers/ps/IOPSKeys.h`：存在电源充电字段，但不构成所有蓝牙设备都能枚举并可靠匹配的证明。
- `/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Headers/hid/IOHIDUsageTables.h`：电池用法定义本身不等于目标设备提供该能力。

现有 system_profiler 适配器只采信地址对应的有效聚合电量。尚无新增来源与目标蓝牙设备身份／充电状态的验证证据，所以未引入 BLE 主动连接或推测充电逻辑。后续应使用具体目标设备确认协议、标识关联、状态转换，再实现充电动画和去重提醒。

## 实机与交付

用户此前已确认耳机重连后的音频正常，本次不重复验证。新增 UI、常驻、权限差异仍需最终安装包实机验收。不同分支合并仍暂停，未集成 test/acceptance、未覆盖当前安装应用。
