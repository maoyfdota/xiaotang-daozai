# 性能页子任务进度

更新：2026-11-26。当前分支 `my_feature_1123`，目标推送 `private/my_feature_1123`。

## 已确认范围

- Mac 性能页：CPU、内存、磁盘读写速率、网络收发速率及近期趋势。
- 页面 ID 10、设置页签 ID 16；蓝牙继续使用 9／15。
- 测试边界：ID 与旧配置迁移、累计计数差值、指标不可用状态。
- 不做进程排行、GPU、常驻样式、Windows 适配或分支合并。

## 实施状态

| 部分 | 状态 | 记录 |
| --- | --- | --- |
| 需求与技术方案 | 已确认 | 用户确认测试边界；Mac 本机 Mach、IOKit 与网卡计数接口已核对 |
| 页面 ID 与配置迁移 | 已完成 | Mac 开放 ID 10／设置页签 16；11 位配置及 8–10 位旧配置迁移用例通过 |
| 页面与设置入口 | 已完成 | 导航、设置分组和性能占位视图已接通；设置映射回归通过 |
| 采样逻辑与趋势 | 已完成 | Mach CPU／内存、IOKit 磁盘、活动网卡累计计数；约 1 秒采样、最多 60 个趋势点；无效值显示“不可用”并保留趋势断点 |
| Mac 页面设置与刷新门控 | 已完成 | 仅性能页打开且启用时采样；蓝牙仍为页面 9／设置页签 15 |
| 综合检查 | 已完成 | 性能单测 6 项、页面配置／Mac 能力／设置映射定向测试通过；Apple Silicon release 检查通过 |
| 中文提交与推送 | 进行中 | 待按私有仓库规则提交，并显式推送到 `private/my_feature_1123` |

## 验证记录

- `cargo test --lib performance -- --test-threads=1`：6 项通过，覆盖累计值差分、首样本、缺失数据源、计数器回退、非法 CPU／内存状态、零间隔与趋势窗口。
- `cargo test --lib performance_page_configuration_migrates_old_lengths_and_round_trips_eleven_entries -- --test-threads=1`：通过。
- `cargo test --lib macos_first_release_exposes_only_local_core_pages -- --test-threads=1`：通过。
- `cargo test --lib settings_groups_keep_every_page_reachable -- --test-threads=1` 与 `every_module_settings_tab_maps_to_its_page`：通过。
- `cargo check --release --target aarch64-apple-darwin --lib`：通过；仍有仓库及 libc Mach 绑定的弃用／未使用警告。
- 宽泛 `cargo test --lib settings -- --test-threads=1`：41 项通过、7 项失败、4 项忽略。失败落在日历、邮件、额度开关迁移、音乐字幕控件和顶栏设置测试；尚未确认这些失败的既有基线，本性能切片不将其标作已解决。
- 更宽的 page 筛选在此前运行中有 15 项失败，未完成归因；本轮只将列出的直接相关定向测试记为通过。

本次采样与趋势代码尚待提交、推送；完成后在此补充提交号和远端状态。
