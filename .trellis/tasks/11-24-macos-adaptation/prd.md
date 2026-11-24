# Mac 适配

## Goal

在 Apple Silicon Mac 上交付可运行的岛仔首期版本，让悬浮岛默认停靠屏幕右侧垂直居中，并支持核心界面与本机数据。

## Background

- 当前产品说明、安装方式和发布流程面向 Windows 10/11，尚未提供 macOS 成品包。
- `src/app.rs` 承载可复用的主要界面与应用状态。
- Windows 主程序使用透明、无边框、不可调整大小并始终置顶的浮动窗口；命中穿透与鼠标交互细节由 Win32 API 实现。
- 系统媒体、通知、蓝牙、剪贴板和凭据存储等模块包含 Windows 专属实现或非 Windows 不支持路径。
- Linux target 配置只为开发和录制提供后端，仓库没有 macOS target 或打包流程。
- 当前开发环境为 Apple Silicon macOS，已安装 `aarch64-apple-darwin` Rust target。
- Rust 官方 target floor 为 Apple Silicon macOS 11.0、Intel x86_64 macOS 10.12；当前环境尚未安装或实测 Intel target。[Rust macOS target requirements](https://doc.rust-lang.org/rustc/platform-support/apple-darwin.html)
- 规划阶段的库检查通过；当时主程序构建因无条件引用 Windows 专属 `system_state` 字段失败，该编译问题已在本任务实现中修正。
- `src/graphics.rs` 当前默认使用 OpenGL，兼容回退路径指定 DX12 后再尝试 Vulkan；尚未定义 macOS 的 Metal 渲染选择。
- 本机数据路径默认从 Windows `APPDATA` 环境变量构造；macOS 下需定义合适的用户数据目录，同时保留 `DAOZAI_DATA_DIR` 覆盖入口。

## Requirements

- 支持 `aarch64-apple-darwin` 并设置 macOS 11.0 作为最低系统版本。
- 首期以可构建、可启动并能使用核心小岛界面与本机数据为目标。
- 保留透明无边框的悬浮岛交互，Mac 首期默认使用屏幕右侧垂直居中的侧边布局，布局尊重可用屏幕范围和刘海安全区域。
- 设置与本机内容保存在当前 macOS 用户的数据目录，跨应用重启保留，并支持 `DAOZAI_DATA_DIR` 测试覆盖。
- 提供可从 Finder 启动的 arm64 `.app` bundle，用于本机验收。
- macOS 程序坞图标在标准画布内留适度透明边距，让 logo 视觉尺寸略小。
- macOS 首版只显示专注、饮水/久坐、待办与标准天气功能页。
- 保留控制本地专注、饮水/久坐和待办提醒的提醒总开关；隐藏 Windows 系统通知方式、提示音、通知读取及聊天软件提醒设置。
- 天气首版使用 Open-Meteo 公共来源。QWeather 凭据设置和请求暂缓。
- 音乐、置物架、邮件、AI 额度、日历和依赖 Windows 或凭据服务的设置在 macOS 首版隐藏。
- 隐藏模块的历史配置和凭据保持原样。启动参数、陈旧配置和直接导航不能重新显示隐藏页面或关联设置，也不能启动隐藏模块的后台服务、轮询、提醒或凭据发现。
- README 说明 Apple Silicon/macOS 11+ 支持范围、首期功能边界及构建运行方式。
- 本任务按 macOS 首期交付验收；Windows 专属能力在 Mac 包中关闭或隐藏，Windows 平台行为不作为本任务验收条件。

## Out of Scope

- Intel x86_64 Mac 支持。
- Mac 首期不交付 Windows 专属系统媒体控制、系统通知、蓝牙状态、全局快捷键、前台应用避让和 Win32 特有托盘行为。
- macOS 菜单栏状态图标、应用更新器、公开下载分发、Developer ID 签名与公证。
- 与服务账号连接相关的 macOS 钥匙串迁移与远程模块完整验收。

## Acceptance Criteria

- [ ] 首期交付核心小岛界面与本机数据，Windows 专属集成后续分期。
- [ ] Mac 首期悬浮岛默认停靠屏幕右侧并垂直居中，展开后仍位于可用屏幕范围内。
- [x] 首期支持 Apple Silicon arm64、macOS 11+。
- [x] 首期隐藏暂缓的系统集成页面与相关设置。
- [ ] `.app` bundle 可在当前 Apple Silicon 开发机启动并显示核心小岛界面。
- [ ] macOS 11.0 deployment target 构建成功，核心设置与本机数据可以保存并在重启后重新载入。
- [ ] 悬浮岛可正常显示、交互和展开/收起，不遮挡菜单栏或刘海安全区域。
- [ ] Mac 首版显示专注、饮水/久坐、待办与 Open-Meteo 天气页面，隐藏音乐、置物架、邮件、AI 额度、日历及其暂缓设置。
- [ ] Mac 设置中可访问本地提醒总开关；系统通知、系统提示、蓝牙和聊天软件提醒控制隐藏，相关历史设置值不被改写。
- [ ] 在默认设置、陈旧的全开配置、`--calendar` 参数和直接页面入口下，隐藏模块仍不可访问，相关后台服务和凭据初始化不会启动，保存的隐藏模块配置保持不变。
- [x] README 中的 Mac 支持范围、首期边界和构建说明与实际能力一致。
- [x] 程序坞图标主体比原始满幅 logo 略小，且 Retina 图标尺寸保持一致。
- [x] 记录构建、运行和交互验证结果，包括尚未通过 GUI 验收的项目。
