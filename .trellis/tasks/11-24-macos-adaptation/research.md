# Mac 适配调研

## 已确认的用户范围

- 首期支持 Apple Silicon arm64，最低 macOS 11。
- Mac 悬浮岛改为默认停靠屏幕右侧垂直居中，布局适配可用屏幕范围和刘海安全区域。
- 首期使用核心界面和本机数据，暂缓的系统集成页面与设置直接隐藏。
- 用户批准创建 Trellis 任务并进入规划，随后批准开始实现；当前任务进入实现与验收阶段。

## 仓库现状

- 分支为 `my_feature_1123`，创建任务时工作区干净。
- `src/app.rs` 承载主要界面、页面切换和设置状态。
- `src/main.rs` 是 eframe 主程序外壳。Windows 显示定位、系统观察、输入监控、通知、托盘和原生窗口行为通过 `#[cfg(windows)]` 分支实现。
- 页面索引定义在 `src/app.rs`，顺序为音乐、专注、健康、置物架、待办、邮件、额度、日历和天气。
- `Settings.visible_pages` 和 `page_order` 位于 `src/model.rs`，格式是固定长度数组并存入 `settings.txt`。
- 当前 `config_dir()` 优先接受 `DAOZAI_DATA_DIR`，否则使用 `%APPDATA%/Daozai`。
- `src/graphics.rs` 默认创建 Glow/OpenGL renderer，兼容回退指定 DX12 后再尝试 Vulkan。
- `vendor/eframe/Cargo.toml` 为 macOS target 启用了 wgpu Metal backend；锁定的 winit 版本为 0.30.13。
- 仓库没有 macOS bundle 或发布脚本，`scripts/package.ps1` 和 `installer/` 仅处理 Windows。

## 编译基线

- 当前宿主为 Apple Silicon macOS 27.2.1，Rust 1.100.0，已安装 `aarch64-apple-darwin` target。
- `rtk cargo check --locked --lib --target aarch64-apple-darwin` 通过。
- `rtk cargo check --locked --bin daozai --target aarch64-apple-darwin` 失败于 `src/main.rs:313` 与 `src/main.rs:332`。两个位置引用了仅在 Windows 配置下声明的 `Shell.system_state`。
- Rust 官方 target 说明列出 `aarch64-apple-darwin` 的最低系统版本为 macOS 11.0，`x86_64-apple-darwin` 为 macOS 10.12。目标版本可以用 `MACOSX_DEPLOYMENT_TARGET` 按二进制单独设置。[Rust Apple Darwin target](https://doc.rust-lang.org/rustc/platform-support/apple-darwin.html)
- 当前设备只能进行 arm64 实机运行验收。macOS 11 兼容性可检查部署目标和旧版 API 可用性，不能在当前环境实机回归。

## macOS 原生能力参考

- winit 0.30 的 macOS 后端支持设置窗口忽略鼠标事件，内部调用 `NSWindow.setIgnoresMouseEvents`。egui 的 `ViewportCommand::MousePassthrough` 可用于动态切换。
- AppKit 的 `NSScreen.safeAreaInsets` 表示刘海摄像头遮挡的区域。
- AppKit `NSWindow.level` 支持 floating 层级，可让窗口显示在普通窗口前。
- eframe 的透明 NativeOptions 可用于 overlay 窗口；项目锁定版本也包含 WGPU 和 Glow renderer。

## 规划审阅发现

- `src/main.rs` 启动时无条件调用 `quota_initialize()` 和 `migrate_merged_switches()`。前者在额度配置不存在时扫描并保存额度配置，后者会迁移邮件/额度开关并可能改写隐藏页面状态。Mac 首版隐藏这两个模块时应跳过这些初始化和迁移。
- `src/main.rs` 无条件创建额度服务和音乐轮询器。轮询开关读取原始 `visible_pages[0]`，而 `Settings::default()` 将所有页面设为可见。仅在界面隐藏音乐页会让陈旧或默认设置继续激活音乐轮询。能力配置需要同时控制服务创建、额度刷新、提醒逻辑和页面可见性。
- `App::new()` 按保存的 `visible_pages` 选择初始页。`App::module_on()` 也只检查保存值。`App::show_page()` 遇到关闭页会打开对应设置分类，因此隐藏页面请求必须先由平台能力拦截，不能落入其设置页。
- `src/main.rs` 无条件处理 `--calendar` 并调用 `open_calendar(None)`。隐藏日历功能需要阻止该参数和直接日历命令恢复页面或服务。
- Mac 二进制当前还在 `src/main.rs` 的 `Env` 构造中引用 Windows 条件字段 `system_state`，导致已记录的两个构建错误。Mac 的网络和音量等值需要使用平台可用状态。
- 非 Windows 的 `open_path()` 当前返回成功但不执行操作，设置页的“打开本地数据目录”按钮会无效。`Cmd::Reveal` 则直接启动 `explorer.exe`，当前调用来自隐藏的置物架。首版应为可见的本机数据目录入口提供 macOS 打开方式，并确保隐藏页面的文件操作不可达。
- 本计划把“标准天气”具体化为 Open-Meteo。QWeather 凭据设置和请求在首版隐藏，既有设置保留但不读取凭据或发起请求。
- AppKit 的屏幕安全区域与 `visibleFrame` 提供不同约束。布局应同时考虑菜单栏可见范围和刘海安全区域；macOS 11 上对安全区域属性做运行时可用性检查，并保留旧系统回退路径。

Sources

- [winit macOS platform API](https://docs.rs/winit/0.30.13/winit/platform/macos/index.html)
- [winit cursor hit-testing API](https://docs.rs/winit/0.30.13/winit/window/struct.Window.html#method.set_cursor_hittest)
- [Apple NSScreen safeAreaInsets](https://developer.apple.com/documentation/appkit/nsscreen/safeareainsets?language=objc)
- [Apple NSScreen visibleFrame](https://developer.apple.com/documentation/appkit/nsscreen/visibleframe)
- [Apple NSWindow level](https://developer.apple.com/documentation/appkit/nswindow/level-swift.property?changes=_1)
- [eframe 0.31.0 documentation](https://docs.rs/crate/eframe/0.31.0)
