# macOS 性能页实施步骤

1. [x] 登记子任务、固定页面 ID 10／设置页签 ID 16，并记录已确认范围和测试边界。
2. [x] 为 Mac 页面能力、旧页面顺序／可见性迁移、计数器差值及不可用显示编写测试。
3. [x] 实现 macOS CPU、内存、磁盘和网络采样及采样历史；逐项让测试转绿。
4. [x] 接入页面与设置入口，绘制四类指标卡和约一分钟趋势，保持蓝牙 ID 9／15。
5. [x] 更新 macOS 平台规范和父任务进度。
6. [x] 运行定向测试、配置回归及 arm64 release 构建；提交 `394aa48` 并显式推送 `private/my_feature_1123`。宽泛 settings/page 测试的未归因失败详见 progress.md。

## 验证命令

- `cargo test --lib performance -- --test-threads=1`
- `cargo test --lib settings -- --test-threads=1`
- `cargo test --lib macos_first_release_exposes_only_local_core_pages -- --test-threads=1`
- `cargo check --release --target aarch64-apple-darwin --lib`
- 完成后按 Trellis 检查阶段复核跨层路由、迁移和平台接口。

## 风险文件

`src/model.rs` 和 `src/app.rs` 的页面数组被导航、隐藏配置及旧配置共用。扩展前先锁定页面 ID 测试；不要改动蓝牙 9／15，也不要对其他分支执行合并。
