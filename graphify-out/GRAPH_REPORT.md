# Graph Report - /Users/maoyufeng/RustStudy/xiaotang-daozai  (2026-11-18)

## 图谱完整性

原始提取中有 810 条关系指向未声明的节点。按有向边计有 1,945 条同端点关系重合，按本次无向图计有 2,041 条。这两项按不同口径统计。缺失端点字段和自环均为 0。全部语义边的端点已核对。

语义提取用量取自 3 个代理的会话计数，输入包含 2,703,104 个缓存 token。

## Corpus Check
- Large corpus: 366 files · ~1,273,411 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder.

## Summary
- 9002 nodes · 23249 edges · 344 communities (330 shown, 14 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 326 edges (avg confidence: 0.81)
- Token cost: 3,217,480 input · 12,876 output

## Graph Freshness
- Built from commit: `f046d6cb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- 界面状态回归测试
- 日历配置与 CalDAV
- 鼠标交互回归测试
- 微信窗口与未读提示
- egui 界面构建
- 额度账户与配置
- egui 上下文读取
- 桌宠表情与动效
- 岛体逐帧绘制
- 本地设置与主题
- 控件交互响应
- HTTP 连接与代理
- 岛体卡片绘制
- 原生 OpenGL 窗口
- Exchange 与日历事件
- 嵌套控件响应
- 侧栏与顶栏面板
- Windows 系统通知
- 应用窗口初始化
- 下拉框与提示
- 原生 wgpu 视口
- 菜单弹窗状态
- 多视口配置
- 控件布局推进
- IMAP 会话命令
- 网易云播放数据
- 富文本与控件文字
- HTTP 跳转与超时测试
- WorkBuddy 登录回调
- 岛体命令与状态
- 滚动区域状态
- Grok 额度查询
- IMAP 正文与能力
- 视口状态管理
- 岛体功能页面
- 浮动区域与图层
- 应用升级下载
- IMAP 命令参数
- 指针输入状态
- 视口样式写入
- 控件命中与交互
- 剪贴板历史
- 资源监测与采样
- egui 窗口配置
- 原生事件循环
- 额度页面与品牌图标
- 上下文默认配置
- 图层与浮动区域
- 平台输出与无障碍
- 日历页面与来源图标
- Windows 输入语言
- 键盘输入状态
- IMAP IDLE 与时间线
- 请求发送与显示布局
- HTTP 客户端配置
- HTTP 请求执行
- 岛体设置控件
- IMAP TLS 连接
- 图像加载器
- 系统媒体控制
- HTTP 首部解析
- HTTP 响应读取
- eframe 应用集成
- 岛体轮廓与停靠
- 歌词来源与匹配
- 界面焦点记忆
- 邮件解析与邀请
- Gemini 额度读取
- 桌面外壳与命令
- IMAP 零拷贝数据
- 岛体通知绘制
- 邮件操作执行
- 控件空间分配
- 文本标签选择
- 原生窗口命中
- 界面堆栈元数据
- IMAP 响应语法
- WorkBuddy 额度查询
- 界面视觉样式
- 逐帧计算缓存
- Claude 用量历史
- 流体皮肤绘制
- 功能截图与像素测试
- 滑块数值输入
- 收件箱后台监听
- 文本光标选区
- HTTP 响应流
- 网页应用运行器
- 颜色选择控件
- 滚动动画设置
- 图像控件构建
- HTTP 连接池
- IMAP 命令构造
- 界面焦点配置
- HTTP 传输错误
- 控件自省工具
- 面板边框与边距
- 缩放平移场景
- 文本编辑缓冲区
- HTTP 模拟传输
- 窗口模糊与合成
- 图像来源与选项
- IMAP 结果解析
- 窗口调整尺寸
- 邮件内容获取
- 网页指针事件
- IMAP 客户端示例
- HTTP 命令行工具
- 浏览器界面适配
- 网格布局状态
- 绘制列表索引
- 文件图标缓存
- 渲染轮次状态
- 按钮控件
- 网络地址解析
- HTTP 类型转换
- HTTP crate 互操作
- 蓝牙设备状态
- 邮件验证码隐私
- Claude 桌面缓存
- 拖动数值控件
- IMAP 解析错误
- eframe 启动错误
- 汽水音乐缓存
- egui 绘图上下文
- 邮件验证码提取
- 常驻栏布局
- 控件样式与间距
- IMAP 信箱数据
- HTTP 中间件
- Rustls 传输
- 键盘修饰键
- 多点触摸状态
- ureq 请求入口
- HTTP 代理设置
- 折叠面板状态
- 网页触摸转换
- DeepSeek 余额查询
- 在线歌词获取
- 可调整尺寸容器
- 图层绘图列表
- 文本撤销历史
- 网页事件监听
- 媒体封面缓存
- 网页 wgpu 绘制
- 托盘菜单构建
- 视口输入与命令
- 酷狗目录缓存
- 中文字体加载
- 网页重绘请求
- 请求体与拖放数据
- 控件矩形索引
- 额度面板状态
- 酷狗曲目信息
- 本机文件存储
- 界面构建参数
- 文本编辑事件
- 邮件本地操作
- 响应体解码
- 时间轴歌词解析
- 网格控件布局
- 文本标签排版
- 自动避让选择器
- 毛玻璃对比度
- 当前媒体选择
- 按类型索引数据
- 控件状态持久化
- 网页文本输入代理
- 播放器进度定位
- 控件标识生成
- HTTP Cookie 管理
- 无窗口演示录制
- 显示器与岛体定位
- 侧边停靠绘制
- 滑块交互绘制
- 文本行与单词选择
- 动画图像帧
- IMAP 元数据扩展
- 自定义 TLS 示例
- 项目说明与外观预览
- Windows 媒体会话查询
- 指针命中测试
- 进度条控件
- IMAP 集成测试
- 软件光栅绘制
- 日历同步调度
- ZCode 额度查询
- 文本编辑状态
- 尺寸调整把手
- 待办提醒选择器
- 弹簧动画插值
- 邮箱账户与服务
- 邮件连接测试
- 额度账户设置
- 网页 OpenGL 绘制
- 逐帧发布缓存
- 折叠标题控件
- 主题样式切换
- 用户数据比较
- 模拟网络流
- 隐私与使用指南
- 原生拖放窗口
- 网页异常处理
- 远程邮件操作反馈
- 托盘窗口消息
- 字体选择与排版
- 模态对话框
- 输入事件过滤
- 数值格式化
- 类型标识映射
- 连接归还读取
- 定长缓存
- 桌面背景采样
- 拖放负载管理
- 托盘状态管理
- 网页日志输出
- HTTP 冒烟测试
- 表单与列表交互测试
- 面板布局收尾
- 默认纹理加载器
- 光标指针交互
- 数值拖动边界
- 图像按钮控件
- 网页画布尺寸监听
- 折叠区域绘制
- 依赖补丁与许可
- 图标图像转换
- 可序列化控件状态
- 岛体状态更新
- 后台轮询时钟
- 事件循环上下文
- 内存字节加载
- 超链接控件
- 分隔线控件
- 加载指示器
- 文本缓冲区修改
- HTTP 请求工厂
- 启动诊断日志
- 酷狗播放器控制
- 邮箱连接端点
- 邮件事件与账户解析
- 应用标题图标
- 字体布局与测量
- 系统 TLS 连接
- 媒体指令工作线程
- 双侧布局控件
- 日程编辑表单
- 番茄钟绘制
- 耗时测量
- 临时与持久状态
- 图形后端回退
- 输入帧初始化
- 图形变换与替换
- 文本光标绘制
- 复选框控件
- 滑块数值映射
- ureq 发布流程
- 蓝牙设备图标
- 显示缩放探针
- 第三方授权归属
- WebGL 上下文创建
- 网页 wgpu 截图
- 调试文本输出
- 菜单栏内部状态
- 文本编辑输出
- IMAP 邮箱名称
- 岛仔版本记录
- Windows 构建与测试
- 歌词内存回归
- 控件动画管理
- 操作系统识别
- 滚动条样式
- HTTP 状态错误
- 窗口边缘响应
- 文本无障碍信息
- 歌词缺失缓存
- 本地 KRC 歌词
- 网页本地存储
- 用户数据序列化
- 指针交互快照
- 界面滚动定位
- 单选按钮控件
- 可选标签控件
- STARTTLS 收件示例
- 邮件阅读与验证码
- 图像纹理载入
- 映射字体文件
- 网页运行器锁
- 常驻组件排序
- 本地凭据解密
- 解码错误消息
- 问题与安全反馈
- 托盘图标加载
- QQ 通知来源
- 界面缩放菜单
- IMAP 引号字符串
- 聊天未读预览
- 会议提醒预览
- 零拷贝迭代
- 跟随系统配色
- 后台服务性能采样
- 上下文调试输出
- 加载器调试输出
- 视口调试输出
- 设备图标授权
- 歌词绘制压力测试
- 切歌内存测试
- 透明窗口探针
- 透明子窗口探针
- 面板折叠响应
- IMAP 版本记录
- 模态关闭响应
- ureq 贡献许可
- 依赖更新策略
- 按键名称映射

## God Nodes (most connected - your core abstractions)
1. `Ui` - 448 edges
2. `Context` - 346 edges
3. `App` - 127 edges
4. `Painter` - 96 edges
5. `WidgetText` - 96 edges
6. `Card` - 89 edges
7. `LayerId` - 88 edges
8. `InnerResponse` - 87 edges
9. `Response` - 79 edges
10. `T` - 68 edges

## Surprising Connections (you probably didn't know these)
- `Classic black, cream, light and dark glass, and floating cloud` --conceptually_related_to--> `Skin`  [INFERRED]
  docs/images/readme/appearance.webp → src/model.rs
- `Top, left and right docking` --conceptually_related_to--> `Dock`  [INFERRED]
  docs/images/readme/docking.webp → src/model.rs
- `support_author()` --references--> `Daozai mascot beside steaming coffee`  [EXTRACTED]
  src/extras.rs → assets/support/daozai-coffee.svg
- `安装、升级与卸载验证` --semantically_similar_to--> `发布前等待 CI`  [INFERRED] [semantically similar]
  .github/workflows/ci.yml → vendor/ureq/RELEASE.txt
- `source_icon()` --references--> `DingTalk white wing on blue`  [EXTRACTED]
  src/calendar_ui.rs → assets/calendar/dingtalk.png

## Import Cycles
- 1-file cycle: `src/input.rs -> src/input.rs`
- 1-file cycle: `src/system.rs -> src/system.rs`
- 1-file cycle: `vendor/egui/src/context.rs -> vendor/egui/src/context.rs`
- 2-file cycle: `vendor/ureq/src/stream.rs -> vendor/ureq/src/unit.rs -> vendor/ureq/src/stream.rs`
- 2-file cycle: `vendor/ureq/src/pool.rs -> vendor/ureq/src/stream.rs -> vendor/ureq/src/pool.rs`
- 2-file cycle: `vendor/ureq/src/agent.rs -> vendor/ureq/src/pool.rs -> vendor/ureq/src/agent.rs`
- 2-file cycle: `src/pet.rs -> src/pet_expressions.rs -> src/pet.rs`
- 3-file cycle: `vendor/ureq/src/agent.rs -> vendor/ureq/src/stream.rs -> vendor/ureq/src/unit.rs -> vendor/ureq/src/agent.rs`
- 3-file cycle: `vendor/ureq/src/agent.rs -> vendor/ureq/src/stream.rs -> vendor/ureq/src/pool.rs -> vendor/ureq/src/agent.rs`
- 4-file cycle: `vendor/ureq/src/agent.rs -> vendor/ureq/src/pool.rs -> vendor/ureq/src/stream.rs -> vendor/ureq/src/unit.rs -> vendor/ureq/src/agent.rs`

## Hyperedges (group relationships)
- **Windows 发布验证** — github_workflows_ci_serial_tests, github_workflows_ci_packaging, github_workflows_ci_installer [EXTRACTED 1.00]
- **ureq 版本发布顺序** — vendor_ureq_release_changelog, vendor_ureq_release_tag, vendor_ureq_release_ci, vendor_ureq_release_publish [EXTRACTED 1.00]

## Communities (344 total, 14 thin omitted)

### Community 0 - "界面状态回归测试"
Cohesion: 0.03
Nodes (152): about_support_preview(), app_notifications_show_temporary_bell_in_minimal_mode(), avoidance_icon_area_stays_one_row_and_scrolls(), battery_module_can_disable_only_the_power_warning(), bluetooth_audio_connection_takes_precedence_over_volume_notice(), bluetooth_badge_follows_connect_and_disconnect(), bluetooth_connection_soon_after_initial_scan_is_not_suppressed(), bluetooth_keyboard_connection_keeps_unrelated_volume_notice() (+144 more)

### Community 1 - "日历配置与 CalDAV"
Cohesion: 0.05
Nodes (93): agent(), basic_auth(), caldav_collection_listing_reads_slash_objects_and_recurring_events(), calendar_collections(), calendar_data(), calendar_data_request(), calendar_home(), Config (+85 more)

### Community 2 - "鼠标交互回归测试"
Cohesion: 0.04
Nodes (102): an_open_page_and_lyrics_can_coexist_with_notifications_and_the_bell(), arrows_and_dots_switch_pages(), battery_module_hides_power_in_collapsed_and_expanded_headers(), blank_todo_draft_closes_when_window_focus_is_lost(), bluetooth_switch_clears_notices_and_restarts_with_fresh_baseline(), browser_minimal_closes_the_open_island_but_still_allows_deliberate_hover(), calendar_dates_are_readable_and_source_labels_do_not_take_a_second_line(), calendar_first_schedule_has_a_large_time_and_keeps_the_lower_list_scrollable() (+94 more)

### Community 3 - "微信窗口与未读提示"
Cohesion: 0.06
Nodes (81): Messenger, Unread, account_main_window(), account_tray_pid(), ancestors(), Burst, button_sized(), class_name() (+73 more)

### Community 4 - "egui 界面构建"
Cohesion: 0.04
Nodes (13): Arc, Color32, Drop, Hsva, Into, Rangef, Response, RwLock (+5 more)

### Community 5 - "额度账户与配置"
Cohesion: 0.06
Nodes (53): Accent, Account, account_credentials(), cadence(), claude_local_config_ignores_retired_reset_fields(), claude_null_window_and_kimi_string_numbers(), codex_week_only_is_a_single_real_window(), codex_window() (+45 more)

### Community 6 - "egui 上下文读取"
Cohesion: 0.06
Nodes (14): Context, ColorImage, FontInsert, Memory, Option, Options, PartialEq, Pos2 (+6 more)

### Community 7 - "桌宠表情与动效"
Cohesion: 0.06
Nodes (59): Mascot eye expression sprite atlas, approach(), atlas_keeps_alpha_and_the_looking_eyes_really_move(), atlas_upload_is_cached_and_light_skins_keep_dark_eyes(), Expression, Expressions, expressions_follow_real_scenarios_and_system_animation_setting(), focus_music_sleep_and_warning_keep_priority() (+51 more)

### Community 8 - "岛体逐帧绘制"
Cohesion: 0.07
Nodes (23): Collapsed island with status rings and clock, Clock alongside colored status rings, Compact status island atop a Windows desktop, Top-centered clock and colored status rings, smoothstep(), App, lyric_key_matches(), LyricsState (+15 more)

### Community 9 - "本地设置与主题"
Cohesion: 0.05
Nodes (35): atomic_write(), atomic_write_replaces_existing_settings(), config_dir(), env_dir(), human_size(), island_appearance_values_are_normalized(), island_size_value(), IslandShape (+27 more)

### Community 10 - "控件交互响应"
Cohesion: 0.05
Nodes (17): Flags, InnerResponse<R>, Response, Align, Arc, BitOr, BitOrAssign, Fn (+9 more)

### Community 11 - "HTTP 连接与代理"
Cohesion: 0.07
Nodes (44): BufReader, ChunkDecoder, TargetAddr, Box<T>, connect_host(), connect_http(), connect_https(), connect_socks() (+36 more)

### Community 12 - "岛体卡片绘制"
Cohesion: 0.09
Nodes (49): alpha(), battery(), bell(), Card, Card<'a>, chase(), check(), checklist() (+41 more)

### Community 13 - "原生 OpenGL 窗口"
Cohesion: 0.06
Nodes (51): HasWindowHandle, NotCurrentContext, PhysicalSize, PossiblyCurrentContext, SwapInterval, change_gl_context(), GlowWinitApp, GlowWinitApp<'app> (+43 more)

### Community 14 - "Exchange 与日历事件"
Cohesion: 0.08
Nodes (63): Failure, calendar_recurring_exceptions_and_all_day_keep_correct_dates(), calendar_weekly_rule_uses_local_weekday_across_utc_midnight(), check_status(), command(), custom(), custom_exchange_completes_policy_handshake_and_reports_partial_compliance(), date() (+55 more)

### Community 15 - "嵌套控件响应"
Cohesion: 0.11
Nodes (12): InnerResponse, Box, FnOnce, Fonts, Frame, Hash, Layout, Memory (+4 more)

### Community 16 - "侧栏与顶栏面板"
Cohesion: 0.10
Nodes (20): lerp(), animate_expansion(), CentralPanel, clamp_to_range(), PanelState, Box, FnOnce, Frame (+12 more)

### Community 17 - "Windows 系统通知"
Cohesion: 0.06
Nodes (51): IAudioEndpointVolume, IDataObject, IMMDevice, audio(), audio_device(), avoidance_matches(), avoidance_window_list_excludes_this_process_and_deduplicates_apps(), avoidance_windows() (+43 more)

### Community 18 - "应用窗口初始化"
Cohesion: 0.06
Nodes (46): AndroidApp, DisplayHandle, EventLoopBuilderHook, HandleError, HasDisplayHandle, RawDisplayHandle, RawWindowHandle, ShaderVersion (+38 more)

### Community 19 - "下拉框与提示"
Cohesion: 0.08
Nodes (46): Text, AboveOrBelow, button_frame(), combo_box_dyn(), ComboBox, paint_default_icon(), Box, Fn (+38 more)

### Community 20 - "原生 wgpu 视口"
Cohesion: 0.07
Nodes (43): create_window(), handle_viewport_output(), initialize_or_update_viewport(), remove_viewports_not_in(), render_immediate_viewport(), ActionRequested, ActiveEventLoop, app (+35 more)

### Community 21 - "菜单弹窗状态"
Cohesion: 0.12
Nodes (29): bar(), context_menu(), context_menu_opened(), menu_button(), menu_custom_button(), menu_image_button(), menu_popup(), MenuResponse (+21 more)

### Community 22 - "多视口配置"
Cohesion: 0.07
Nodes (27): CursorGrab, epaint::ColorImage, IconData, IMEPurpose, ImmediateViewport, ResizeDirection, Arc, Box (+19 more)

### Community 23 - "控件布局推进"
Cohesion: 0.09
Nodes (12): Direction, Layout, Region, Align, Align2, Default, Pos2, Rect (+4 more)

### Community 24 - "IMAP 会话命令"
Cohesion: 0.11
Nodes (16): B, S1, S2, Connection<T>, DateTime, FixedOffset, HashSet, Into (+8 more)

### Community 25 - "网易云播放数据"
Cohesion: 0.08
Nodes (46): control(), control_hotkey(), cover_failure_can_retry_without_a_minute_of_blank_artwork(), cover_url(), CoverCache, decode(), encode(), event() (+38 more)

### Community 26 - "富文本与控件文字"
Cohesion: 0.08
Nodes (13): RichText, Box, Color32, Cow, Default, FontFamily, FontId, Fonts (+5 more)

### Community 27 - "HTTP 跳转与超时测试"
Cohesion: 0.05
Nodes (37): builder(), cookie_and_redirect(), dirty_streams_not_returned(), idle_timeout_handler(), idle_timeout_handler_408(), Error, Result, TcpStream (+29 more)

### Community 28 - "WorkBuddy 登录回调"
Cohesion: 0.06
Nodes (38): clean(), parse_paste(), plausible(), Balance, Vec, Wallet, challenge(), connect() (+30 more)

### Community 29 - "岛体命令与状态"
Cohesion: 0.07
Nodes (35): 代码导航与模块分工, blur_follows_the_island_outline_not_its_bounding_box(), blur_pieces(), CalendarFeed, CalendarForm, CalendarState, Cmd, collapsing_pill_blur_radius_fits_the_physical_bounds() (+27 more)

### Community 30 - "滚动区域状态"
Cohesion: 0.09
Nodes (19): Prepared, Box, Default, FnOnce, Hash, Id, Into, Option (+11 more)

### Community 31 - "Grok 额度查询"
Cohesion: 0.09
Nodes (51): Map, account_slots_survive_string_wrapping_and_do_not_follow_the_active_slot(), accounts(), checksum(), claims(), Connected, credential(), decrypt() (+43 more)

### Community 32 - "IMAP 正文与能力"
Cohesion: 0.07
Nodes (43): BodyExt1Part, BodyExtMPart, BodyFields, mock_body_text(), BodyParams, Option, Vec, test_body_structure_message() (+35 more)

### Community 33 - "视口状态管理"
Cohesion: 0.06
Nodes (26): ImageLoadResult, PaintStats, ContextImpl, RequestRepaintInfo, Arc, Box, BTreeMap, BytesLoadResult (+18 more)

### Community 34 - "岛体功能页面"
Cohesion: 0.07
Nodes (34): 25-minute focus timer with focus and break controls, Expanded Pomodoro timer, Expanded music player controls, Track progress, playback buttons and lyrics toggle, Daily timeline, Agenda, weekly tasks, focus and hydration panels, Weekly repeating tasks, Pomodoro timer (+26 more)

### Community 35 - "浮动区域与图层"
Cohesion: 0.08
Nodes (16): Area, AreaState, automatic_area_position(), pointer_pressed_on_area(), Prepared, Align2, Default, Id (+8 more)

### Community 36 - "应用升级下载"
Cohesion: 0.08
Nodes (43): background_result_and_disconnect_allow_retry(), cancellation_before_install_handoff_keeps_the_app_open(), check(), clip_update_window(), dismissing_update_dialog_cancels_download_and_keeps_release_available(), download(), download_rejects_truncation_corruption_excess_and_cancellation(), Event (+35 more)

### Community 37 - "IMAP 命令参数"
Cohesion: 0.07
Nodes (48): authenticate(), capability(), check(), close(), copy(), create(), create_command(), delete() (+40 more)

### Community 38 - "指针输入状态"
Cohesion: 0.07
Nodes (8): PointerButton, Click, PointerEvent, PointerState, Duration, Option, Pos2, Vec2

### Community 39 - "视口样式写入"
Cohesion: 0.10
Nodes (7): FnOnce, Id, R, Response, TessellationOptions, GraphicLayers, IdMap

### Community 40 - "控件命中与交互"
Cohesion: 0.07
Nodes (10): IdSource, Debug, Formatter, Result, Self, Sense, register_rect(), Id (+2 more)

### Community 41 - "剪贴板历史"
Cohesion: 0.09
Nodes (31): MutexGuard, Clip, excluded(), History, history_copy_preserves_order_but_external_copy_promotes(), history_dedupes_caps_and_keeps_pins(), pinned_round_trip_and_preview(), preview() (+23 more)

### Community 42 - "资源监测与采样"
Cohesion: 0.08
Nodes (33): AtomicUsize, append(), append_log(), append_rotating(), blocked_jobs_have_a_process_wide_bound_and_release_their_slots(), cancellation_does_not_release_a_still_running_request(), diagnostic_history_rotates_and_rejects_oversized_records(), feature_logs_rotate_and_cap_each_line() (+25 more)

### Community 43 - "egui 窗口配置"
Cohesion: 0.10
Nodes (8): Align2, Fn, Into, Pos2, Self, Vec2, Vec2b, Window<'open>

### Community 44 - "原生事件循环"
Cohesion: 0.08
Nodes (36): ApplicationHandler, StartCause, create_event_loop(), ActiveEventLoop, AppCreator, DeviceEvent, DeviceId, Error (+28 more)

### Community 45 - "额度页面与品牌图标"
Cohesion: 0.07
Nodes (39): Claude radial sunburst, Codex interlocking knot, DeepSeek whale, Gemini four-pointed star, Grok Bot face with two slanted eyes, Kimi letter K, WorkBuddy multicolour circular emblem, ZCode circular Z (+31 more)

### Community 46 - "上下文默认配置"
Cohesion: 0.08
Nodes (19): ContextCallback, TextureManager, NamedContextCallback, Plugins, RepaintCause, ClippedPrimitive, ClippedShape, Cow (+11 more)

### Community 47 - "图层与浮动区域"
Cohesion: 0.08
Nodes (18): OrderMap, LayerId, Debug, Formatter, Id, Result, Self, Areas (+10 more)

### Community 48 - "平台输出与无障碍"
Cohesion: 0.08
Nodes (23): TreeUpdate, Fn, ViewportIdSet, FullOutput, OpenUrl, OutputCommand, OutputEvent, PlatformOutput (+15 more)

### Community 49 - "日历页面与来源图标"
Cohesion: 0.12
Nodes (32): DingTalk white wing on blue, Feishu blue and green bird, Purple envelope for email calendar invitations, WeCom blue speech bubble, Calendar grid outline, Selected date and upcoming appointment timeline, Monthly calendar and daily agenda, App (+24 more)

### Community 50 - "Windows 输入语言"
Cohesion: 0.10
Nodes (32): HHOOK, active_profile(), actual_switch_keys_require_a_matching_state_change_and_are_used_once(), changing_app_windows_synchronizes_their_modes_without_notifying(), changing_focused_controls_does_not_reuse_another_controls_mode(), chinese(), ctrl_shift_detects_tsf_profile_changes_even_when_the_layout_is_unchanged(), ctrl_shift_detects_two_layouts_with_the_same_language_id() (+24 more)

### Community 51 - "键盘输入状态"
Cohesion: 0.07
Nodes (17): Action, format_kb_shortcut(), KeyboardShortcut, ModifierNames, Key, Option, Self, test_key_from_name() (+9 more)

### Community 52 - "IMAP IDLE 与时间线"
Cohesion: 0.09
Nodes (29): Prefixed<Stream>, discover(), Found, kugou_position(), name(), Process, query(), read() (+21 more)

### Community 53 - "请求发送与显示布局"
Cohesion: 0.11
Nodes (30): Display, can_propagate_authorization_on_redirect(), cleartext_http_proxy_keeps_absolute_form(), connect(), connect_inner(), connect_socket(), extract_cookies(), https_http_proxy_uses_origin_form_after_connect() (+22 more)

### Community 54 - "HTTP 客户端配置"
Cohesion: 0.09
Nodes (20): agent_config_debug(), agent_implements_send_and_sync(), AgentBuilder, AgentConfig, AgentState, RedirectAuthHeaders, Arc, Box (+12 more)

### Community 55 - "HTTP 请求执行"
Cohesion: 0.10
Nodes (16): Request, request_implements_send_and_sync(), RequestUrl, Debug, Duration, Formatter, Option, P (+8 more)

### Community 56 - "岛体设置控件"
Cohesion: 0.08
Nodes (40): QQ penguin with red scarf, Folder with check mark, Circled information mark, Musical note outline, Slider controls outline, Gray information settings badge, Orange focus clock settings badge, Gray general settings gear (+32 more)

### Community 57 - "IMAP TLS 连接"
Cohesion: 0.09
Nodes (34): BufStream, U, connect_all_timeout(), connect_timeout(), main(), A, Box, Duration (+26 more)

### Community 58 - "图像加载器"
Cohesion: 0.09
Nodes (32): BytesLoaderImpl, ImageLoaderImpl, TextureLoaderImpl, Bytes, BytesLoader, BytesPoll, ImageLoader, ImagePoll (+24 more)

### Community 59 - "系统媒体控制"
Cohesion: 0.08
Nodes (33): CO_MTA_USAGE_COOKIE, artwork_decodes_png_jpeg_webp_and_rejects_invalid_bytes(), browser_video_authors_are_not_enough_to_identify_a_song(), control(), decode_artwork(), init_winrt(), kugou_music_without_artist_is_still_a_song(), live_client_transport() (+25 more)

### Community 60 - "HTTP 首部解析"
Cohesion: 0.09
Nodes (22): get_all_headers(), get_header(), get_header_raw(), has_header(), Header, HeaderLine, is_tchar(), Debug (+14 more)

### Community 61 - "HTTP 响应读取"
Cohesion: 0.09
Nodes (23): body_type(), charset(), connection_option(), gzip_content_length(), into_string_large(), LimitedRead<R>, parse_header_with_non_utf8(), parse_simple_json() (+15 more)

### Community 62 - "eframe 应用集成"
Cohesion: 0.08
Nodes (34): EventResponse, eframe default icon with a white lowercase e, White e on a dark rounded square, apply_window_settings(), create_storage(), create_storage_with_file(), EpiIntegration, largest_monitor_point_size() (+26 more)

### Community 63 - "岛体轮廓与停靠"
Cohesion: 0.13
Nodes (36): FillKey, Mesh, Dock, animated_outlines_do_not_accumulate_old_meshes(), densify(), edge_normal(), EdgeMap, fill() (+28 more)

### Community 64 - "歌词来源与匹配"
Cohesion: 0.08
Nodes (36): artist_set(), clean_title(), fallback_checks_identity_even_when_wrong_recording_has_exact_duration(), fetch_from(), fetch_qqmusic(), http_search_and_network_errors_are_distinct_from_missing_lyrics(), live_kugou_short_pause_stays_on_previous_line(), live_kugou_uses_native_timestamps() (+28 more)

### Community 65 - "界面焦点记忆"
Cohesion: 0.11
Nodes (8): InteractionState, Memory, FontDefinitions, FontInsert, Id, Options, ViewportIdMap, ViewportIdSet

### Community 66 - "邮件解析与邀请"
Cohesion: 0.09
Nodes (34): ParsedMail, accounts_path(), accounts_roundtrip_without_secrets(), all_day_calendar_event_has_exclusive_end(), body_of(), calendar_part_is_collected_from_mime(), code_of(), custom_config_roundtrips_without_a_secret() (+26 more)

### Community 67 - "Gemini 额度读取"
Cohesion: 0.14
Nodes (38): bridge(), browser_session(), Connect, Connected, cookie(), decode(), email(), fetch() (+30 more)

### Community 68 - "桌面外壳与命令"
Cohesion: 0.08
Nodes (34): ExitCode, Monitor, Service, background_repaint_keeps_the_requested_frame_interval(), cursor_logical(), default_apps_urls_and_other_errors_never_open_the_file_picker(), error(), local_now() (+26 more)

### Community 69 - "IMAP 零拷贝数据"
Cohesion: 0.08
Nodes (24): Ord, PartialOrd, Flag<'a>, Flag<'static>, Box, D, Debug, Deref (+16 more)

### Community 70 - "岛体通知绘制"
Cohesion: 0.09
Nodes (13): Alert, App, bluetooth_notice_content(), notice_badge(), notice_icon(), parse_due(), Cmd, Into (+5 more)

### Community 71 - "邮件操作执行"
Cohesion: 0.12
Nodes (37): change(), changed_uid_validity_blocks_all_writes(), Completed, execute(), fail(), Folder, live_mail_management_capabilities(), local_trash_reload_respects_uid_validity_and_reuses_bodies() (+29 more)

### Community 72 - "控件空间分配"
Cohesion: 0.09
Nodes (9): Placer, Layout, Option, Pos2, Rect, Region, Self, ToString (+1 more)

### Community 73 - "文本标签选择"
Cohesion: 0.11
Nodes (26): CurrentSelection, estimate_row_height(), got_copy_event(), LabelSelectionState, pos_in_galley(), process_selection_key_events(), Arc, CCursor (+18 more)

### Community 74 - "原生窗口命中"
Cohesion: 0.10
Nodes (31): Backend, activation_keeps_focus_messages_but_suppresses_nonclient_paint(), apply_style(), compositor_blur_allowed(), editing_mode_does_not_recalculate_the_native_window_frame(), enable_alpha_composition(), expanding_raises_the_island_without_activating_it(), file_capture_below_the_island_still_returns_click_through() (+23 more)

### Community 75 - "界面堆栈元数据"
Cohesion: 0.11
Nodes (19): FusedIterator, Any, Arc, Frame, HashMap, Id, Into, Iterator (+11 more)

### Community 76 - "IMAP 响应语法"
Cohesion: 0.08
Nodes (29): ParseResult, astring_char(), atom_char(), atom_specials(), list_wildcards(), quoted_data(), quoted_specials(), resp_specials() (+21 more)

### Community 77 - "WorkBuddy 额度查询"
Cohesion: 0.13
Nodes (34): api(), Auth, Connect, Connected, Failure, fetch(), forget(), local_snapshot() (+26 more)

### Community 78 - "界面视觉样式"
Cohesion: 0.09
Nodes (16): &mut CornerRadius, &mut crate::Frame, &mut Margin, &mut Shadow, &mut Stroke, NumericColorSpace, Color32, CornerRadius (+8 more)

### Community 79 - "逐帧计算缓存"
Cohesion: 0.06
Nodes (23): Computer, CacheStorage, Box, Cache, Clone, Debug, Formatter, HashMap (+15 more)

### Community 80 - "Claude 用量历史"
Cohesion: 0.16
Nodes (38): account(), bound_desktop_login_does_not_adopt_a_peer_record_when_switching_back(), code_fixture(), code_identity(), code_path(), code_slot(), desktop_identity(), desktop_record() (+30 more)

### Community 81 - "流体皮肤绘制"
Cohesion: 0.13
Nodes (23): base_is_neutral_gray_not_a_neon_wash(), bloom_is_gone_so_animating_stops_after_the_fade(), chase_light_follows_the_head_and_is_soft(), collapsed_is_translucent_and_expanded_fades_toward_the_bottom(), color_returns_to_neutral_when_music_stops(), compose_alpha_stays_within_bounds(), cursor_glow_fades_in_and_out(), cursor_glow_is_smooth_across_neighbouring_samples() (+15 more)

### Community 82 - "功能截图与像素测试"
Cohesion: 0.08
Nodes (36): adding_a_todo_saves_content_then_offers_an_optional_reminder(), automatic_minimal_hides_unread_hit_area_until_reopened(), calendar_add_button_does_not_overlap_the_battery_ring(), claude_local_quota_shows_the_record_time_and_does_not_invent_resets(), empty_todo_draft_can_be_cancelled_by_clicking_elsewhere(), feature_gallery(), feature_raster(), feature_raster_sized() (+28 more)

### Community 83 - "滑块数值输入"
Cohesion: 0.12
Nodes (9): Color32, Fn, FnMut, Into, Num, Option, Self, ToString (+1 more)

### Community 84 - "收件箱后台监听"
Cohesion: 0.13
Nodes (35): AtomicU64, bursts_do_not_skip_messages_between_batches(), collect(), cooldown_wait_can_be_cancelled(), Event, fetch_new(), inbox(), initial_mail_is_silent_and_ready_follows_successful_reading() (+27 more)

### Community 85 - "文本光标选区"
Cohesion: 0.11
Nodes (17): PCursor, TextPosition, ccursor_from_accesskit_text_position(), CCursorRange, CursorRange, move_single_cursor(), PCursorRange, CCursor (+9 more)

### Community 86 - "HTTP 响应流"
Cohesion: 0.09
Nodes (18): charset_from_content_type(), Compression, ConnectionOption, ErrorReader, history(), LimitedRead, Response, Box (+10 more)

### Community 87 - "网页应用运行器"
Cohesion: 0.08
Nodes (18): ActiveWebPainter, AppRunner, LocalStorage, App, AppCreator, Arc, Box, ClippedPrimitive (+10 more)

### Community 88 - "颜色选择控件"
Cohesion: 0.17
Nodes (35): HsvaGamma, Rgba, Alpha, background_checkers(), color_button(), color_cache_get(), color_cache_set(), color_edit_button_hsva() (+27 more)

### Community 89 - "滚动动画设置"
Cohesion: 0.12
Nodes (10): DebugOptions, Interaction, &mut FontTweak, Default, Rangef, Self, ScrollAnimation, Selection (+2 more)

### Community 90 - "图像控件构建"
Cohesion: 0.13
Nodes (9): Image<'a>, ImageSource<'a>, ImageSource<'static>, Color32, CornerRadius, Cow, From, Into (+1 more)

### Community 91 - "HTTP 连接池"
Cohesion: 0.15
Nodes (21): ConnectionPool, Inner, pool_checks_proxy(), pool_connections_limit(), pool_per_host_connections_limit(), PoolKey, PoolReturner, PoolReturnRead (+13 more)

### Community 92 - "IMAP 命令构造"
Cohesion: 0.10
Nodes (18): find_rc(), main(), Option, PathBuf, Copy, Command, CommandBuilder, FetchBuilderAttributes (+10 more)

### Community 93 - "界面焦点配置"
Cohesion: 0.10
Nodes (13): Focus, FocusDirection, FocusWidget, Options, order_map_total_ordering(), Arc, Default, IdMap (+5 more)

### Community 94 - "HTTP 传输错误"
Cohesion: 0.13
Nodes (17): connection_closed(), Error, error_implements_send_and_sync(), ErrorKind, io_error(), Box, From, Into (+9 more)

### Community 95 - "控件自省工具"
Cohesion: 0.08
Nodes (26): AllocInfo, &epaint::stats::PaintStats, font_family_ui(), font_id_ui(), font_texture_ui(), label(), &memory::InteractionState, &mut epaint::TessellationOptions (+18 more)

### Community 96 - "面板边框与边距"
Cohesion: 0.19
Nodes (12): Marginf, Frame, Box, Color32, CornerRadius, FnOnce, Into, Margin (+4 more)

### Community 97 - "缩放平移场景"
Cohesion: 0.10
Nodes (20): FnOnce, R, fit_to_rect_in_scene(), Default, FnOnce, Into, R, Rangef (+12 more)

### Community 98 - "文本编辑缓冲区"
Cohesion: 0.13
Nodes (7): Align, Color32, Hash, Into, Self, TextEdit<'t>, TextBuffer

### Community 99 - "HTTP 模拟传输"
Cohesion: 0.10
Nodes (21): make_response(), Recorder, resolve_handler(), Arc, Box, Debug, Error, Formatter (+13 more)

### Community 100 - "窗口模糊与合成"
Cohesion: 0.10
Nodes (27): CompositionBrush, CompositionRoundedRectangleGeometry, Compositor, ContainerVisual, DesktopWindowTarget, DispatcherQueueController, SpriteVisual, BlurPiece (+19 more)

### Community 101 - "图像来源与选项"
Cohesion: 0.14
Nodes (18): Rot2, Image, ImageFit, ImageOptions, ImageSize, ImageSource, paint_texture_at(), paint_texture_load_result() (+10 more)

### Community 102 - "IMAP 结果解析"
Cohesion: 0.15
Nodes (32): handle_unilateral(), MapOrNot, parse_authenticate_response(), parse_capabilities(), parse_capabilities_w_unilateral(), parse_capability_case_insensitive_test(), parse_capability_invalid_test(), parse_capability_test() (+24 more)

### Community 103 - "窗口调整尺寸"
Cohesion: 0.16
Nodes (21): Prepared, close_button(), move_and_resize_window(), paint_frame_interaction(), paint_resize_corner(), PossibleInteractions, resize_interaction(), resize_response() (+13 more)

### Community 104 - "邮件内容获取"
Cohesion: 0.11
Nodes (25): account(), item(), large_binary_attachment_is_never_requested_and_full_text_is_kept(), legacy(), literal(), live_segment_fetch_matches_legacy_for_recent_mail(), nested_alternatives_preserve_html_links_encoding_and_calendar(), parts() (+17 more)

### Community 105 - "网页指针事件"
Cohesion: 0.26
Nodes (31): get_display_size(), install_blur_focus(), install_color_scheme_change_event(), install_copy_cut_paste(), install_dpr_change_event(), install_drag_and_drop(), install_event_handlers(), install_keydown() (+23 more)

### Community 106 - "IMAP 客户端示例"
Cohesion: 0.07
Nodes (26): IMAP 跨平台流水线, GreenMail 测试容器, IMAP 多系统 Rust 测试矩阵, IMAP FreeBSD 检查, FreeBSD 构建与测试, GmailOAuth2, Response, 基础 IMAP 连接示例 (+18 more)

### Community 107 - "HTTP 命令行工具"
Cohesion: 0.12
Nodes (22): CertificateDer, ClientConfig, DigitallySignedStruct, HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier, ServerName, SignatureScheme (+14 more)

### Community 108 - "浏览器界面适配"
Cohesion: 0.11
Nodes (28): ClipboardItem, MediaQueryList, canvas_size_in_points(), create_clipboard_item(), cursor_web_name(), focused_element(), has_focus(), native_pixels_per_point() (+20 more)

### Community 109 - "网格布局状态"
Cohesion: 0.13
Nodes (11): ColorPickerFn, GridLayout, Arc, Color32, Option, Rect, Region, Vec (+3 more)

### Community 110 - "绘制列表索引"
Cohesion: 0.18
Nodes (13): PathStroke, StrokeKind, ShapeIdx, Align2, Color32, CornerRadius, Into, Pos2 (+5 more)

### Community 111 - "文件图标缓存"
Cohesion: 0.14
Nodes (25): PCWSTR, ResultItem, Cache, file_icon(), load_preview(), Loaded, oversized_shell_thumbnails_are_reduced_before_upload(), Preview (+17 more)

### Community 112 - "渲染轮次状态"
Cohesion: 0.12
Nodes (19): AccessKitPassState, DebugRect, PassState, PerLayerState, PerWidgetTooltipState, Align, Default, HashMap (+11 more)

### Community 113 - "按钮控件"
Cohesion: 0.14
Nodes (11): Button, Button<'a>, Color32, CornerRadius, Into, Option, Response, Self (+3 more)

### Community 114 - "网络地址解析"
Cohesion: 0.08
Nodes (21): Ipv6Resolver, Result, SocketAddr, Vec, ArcResolver, F, Resolver, Arc (+13 more)

### Community 115 - "HTTP 类型转换"
Cohesion: 0.12
Nodes (16): add_header(), convert_to_http_request_builder_with_invalid_utf8_header(), convert_to_http_request_builder_with_invalid_utf8_header(), convert_to_http_response_builder_with_invalid_utf8_header(), convert_to_http_response_string(), create_builder(), http::request::Builder, http::Response<Box<dyn Read + Send + Sync + 'static>> (+8 more)

### Community 116 - "HTTP crate 互操作"
Cohesion: 0.12
Nodes (15): convert_to_http_response_builder_with_invalid_utf8_header(), convert_to_http_response_string(), create_builder(), http::request::Builder, http::Response<Box<dyn Read + Send + Sync + 'static>>, http::Response<String>, http::Response<Vec<u8>>, Request (+7 more)

### Community 117 - "蓝牙设备状态"
Cohesion: 0.14
Nodes (24): DEVPROPKEY, HDEVINFO, SP_DEVINFO_DATA, bluetooth_connection_change_refreshes_immediately_and_failed_scans_back_off(), byte_prop(), Cache, Device, device_kind() (+16 more)

### Community 118 - "邮件验证码隐私"
Cohesion: 0.12
Nodes (17): Link, Item, delay_history_reconnect_and_manual_reveal_never_extend_auto_visibility(), explicit_lifetime(), fallback_is_configurable_and_masks_formatted_codes(), item(), key(), mask() (+9 more)

### Community 119 - "Claude 桌面缓存"
Cohesion: 0.17
Nodes (25): body(), Cache, desktop_cache_decodes_compressed_responses_and_rejects_damage_or_excessive_output(), desktop_cache_keeps_missing_expired_and_unzoned_resets_unknown(), desktop_cache_reads_files_held_open_with_delete_access(), desktop_cache_reads_official_reset_times_without_changing_files(), desktop_cache_rejects_other_organizations_and_old_or_future_responses(), desktop_cache_supports_limit_rows_and_never_infers_missing_windows() (+17 more)

### Community 120 - "拖动数值控件"
Cohesion: 0.16
Nodes (9): DragValue<'a>, Fn, FnMut, Into, Num, RangeInclusive, Response, Self (+1 more)

### Community 121 - "IMAP 解析错误"
Cohesion: 0.15
Nodes (17): BufError, DecodeError, IoError, StdError, TlsError, TlsHandshakeError, Utf8Error, Error (+9 more)

### Community 122 - "eframe 启动错误"
Cohesion: 0.10
Nodes (23): ConfigTemplate, EventLoopError, PainterError, enlarged_root_uses_physical_font_pixels_with_settings_open(), Error, root_input_is_converted_into_scaled_points(), AppCreator, Box (+15 more)

### Community 123 - "汽水音乐缓存"
Cohesion: 0.16
Nodes (17): decode(), Decoder, Decoder<'a>, entries(), fingerprint(), le(), live_cached_metadata_variants(), path() (+9 more)

### Community 124 - "egui 绘图上下文"
Cohesion: 0.12
Nodes (7): Painter, ClippedShape, FnMut, Option, Rect, Self, Vec2

### Community 125 - "邮件验证码提取"
Cohesion: 0.14
Nodes (20): Regex, after(), around(), before(), candidate(), clock(), Code, emphasized() (+12 more)

### Community 126 - "常驻栏布局"
Cohesion: 0.15
Nodes (14): animations_enabled(), clock_12h_keeps_components_separate_in_custom_orders(), custom_order_preserves_groups_and_hidden_positions(), deepseek_balance_follows_more_and_never_overlaps(), every_component_combination_fits_without_overlaps(), Layout, Motion, quota_slot() (+6 more)

### Community 127 - "控件样式与间距"
Cohesion: 0.13
Nodes (15): default_text_styles(), Align, BTreeMap, FontId, Margin, Option, Rect, TextWrapMode (+7 more)

### Community 128 - "IMAP 信箱数据"
Cohesion: 0.10
Nodes (16): Fetch, DateTime, FixedOffset, Option, Seq, Uid, Vec, Mailbox (+8 more)

### Community 129 - "HTTP 中间件"
Cohesion: 0.12
Nodes (22): CounterMiddleware, CounterState, main(), Arc, Error, Mutex, Request, Response (+14 more)

### Community 130 - "Rustls 传输"
Cohesion: 0.11
Nodes (19): ClientConnection, RootCertStore, StreamOwned, TryFrom, Arc<rustls::ClientConfig>, default_tls_config(), is_close_notify(), root_certs() (+11 more)

### Community 131 - "键盘修饰键"
Cohesion: 0.13
Nodes (7): Modifiers, BitOr, BitOrAssign, Debug, Formatter, Result, Self

### Community 132 - "多点触摸状态"
Cohesion: 0.16
Nodes (15): TouchDeviceId, ActiveTouch, DynGestureState, GestureState, MultiTouchInfo, PinchType, BTreeMap, Debug (+7 more)

### Community 133 - "ureq 请求入口"
Cohesion: 0.13
Nodes (25): 阻塞式 HTTP 客户端, ureq HTTP 客户端说明, HTTP 代理与 CONNECT 隧道, HTTPS 与 TLS 后端, agent(), connect_http_google(), connect_https_google_native_tls(), connect_https_google_rustls() (+17 more)

### Community 134 - "HTTP 代理设置"
Cohesion: 0.16
Nodes (17): parse_proxy_http_user_pass_server_port(), parse_proxy_http_user_pass_server_port_trailing_slash(), parse_proxy_server(), parse_proxy_server_port(), parse_proxy_socks4_user_pass_server_port(), parse_proxy_socks4a_user_pass_server_port(), parse_proxy_socks5_user_pass_server_port(), parse_proxy_socks_user_pass_server_port() (+9 more)

### Community 135 - "折叠面板状态"
Cohesion: 0.16
Nodes (9): HeaderRet, CollapsingState, HeaderResponse, HeaderResponse<'_, HeaderRet>, InnerState, paint_default_icon(), Prepared, Response (+1 more)

### Community 136 - "网页触摸转换"
Cohesion: 0.13
Nodes (24): MouseEvent, Touch, TouchEvent, on_keydown(), on_keyup(), KeyboardEvent, should_prevent_default_for_key(), button_from_mouse_event() (+16 more)

### Community 137 - "DeepSeek 余额查询"
Cohesion: 0.16
Nodes (20): Connect, Connected, credential(), deepseek_live_read_only_diagnostic(), deepseek_requires_valid_amounts_and_preserves_currency(), fetch(), forget(), load() (+12 more)

### Community 138 - "在线歌词获取"
Cohesion: 0.21
Nodes (23): fetch(), fetch_for_display(), fetch_for_source(), fetch_netease(), fetch_netease_song(), fetch_online(), fetch_online_detail(), fetch_with_sources() (+15 more)

### Community 139 - "可调整尺寸容器"
Cohesion: 0.18
Nodes (7): Resize, Default, Hash, Into, Self, Vec2, Vec2b

### Community 140 - "图层绘图列表"
Cohesion: 0.13
Nodes (11): PaintList, ClippedShape, ExactSizeIterator, FnOnce, HashMap, I, Option, Rect (+3 more)

### Community 141 - "文本撤销历史"
Cohesion: 0.14
Nodes (13): Flux, Debug, Default, Formatter, Option, Result, Self, State (+5 more)

### Community 142 - "网页事件监听"
Cohesion: 0.16
Nodes (15): AddEventListenerOptions, AnimationFrameRequest, EventToUnsubscribe, IntervalHandle, AppCreator, Closure, E, Event (+7 more)

### Community 143 - "媒体封面缓存"
Cohesion: 0.17
Nodes (21): ArtworkRequest, GlobalSystemMediaTransportControlsSessionManager, artwork_cache_replaces_a_temporary_cover_on_the_same_song(), artwork_cache_retries_late_images_and_clears_on_song_change(), ArtworkCache, ArtworkWorker, changing_tracks_reuses_the_cover_worker_without_waiting_for_old_backoff(), cover_bytes() (+13 more)

### Community 144 - "网页 wgpu 绘制"
Cohesion: 0.12
Nodes (17): CaptureReceiver, CaptureSender, CaptureState, SurfaceConfiguration, SurfaceError, SurfaceErrorAction, TextureFormat, TextureView (+9 more)

### Community 145 - "托盘菜单构建"
Cohesion: 0.13
Nodes (15): HMENU, about(), Event, fill_tip(), labels(), menu_items(), menu_order_checks_and_separator(), popup_menu() (+7 more)

### Community 146 - "视口输入与命令"
Cohesion: 0.11
Nodes (17): process_viewport_commands(), ActionRequested, HashSet, IntoIterator, Window, DroppedFile, HoveredFile, RawInput (+9 more)

### Community 147 - "酷狗目录缓存"
Cohesion: 0.17
Nodes (22): alias_header_requires_exact_filename_and_opening_credit(), cached_duet_identity_ignores_order_and_credit_field(), configured_unicode_directory_and_exact_track_selection(), decode(), directory_from_ini(), duet_cache_accepts_primary_artist_only_when_filename_matches_full_track(), empty_headers_use_exact_filename_without_accepting_other_versions(), fixture() (+14 more)

### Community 148 - "中文字体加载"
Cohesion: 0.14
Nodes (22): alpha_blue(), app_icon(), build_fonts(), Face, find(), font(), font_bytes(), font_data_is_shared_across_contexts_without_owned_byte_copies() (+14 more)

### Community 149 - "网页重绘请求"
Cohesion: 0.12
Nodes (14): NeedRepaint, parse_query_map(), BTreeMap, Default, Mutex, Option, Self, Vec (+6 more)

### Community 150 - "请求体与拖放数据"
Cohesion: 0.14
Nodes (16): BodySize, copy_chunked(), Payload, Payload<'a>, Box, Debug, Default, Formatter (+8 more)

### Community 151 - "控件矩形索引"
Cohesion: 0.15
Nodes (12): HashMap, Id, IdMap, Iterator, Option, PartialEq, Rect, Self (+4 more)

### Community 152 - "额度面板状态"
Cohesion: 0.12
Nodes (17): Connect, Login, claude_local_quota_displays_only_recorded_reset_times(), claude_local_records_do_not_restore_old_online_snapshots_or_claim_a_new_update(), claude_verified_history_survives_client_exit_and_island_restart(), deepseek_refresh_failure_keeps_balance_until_success(), quota_live_cold_start(), restart_restores_last_balance_only_for_the_bound_account() (+9 more)

### Community 153 - "酷狗曲目信息"
Cohesion: 0.16
Nodes (11): Cache, current(), Process, read(), Region, Drop, Instant, Option (+3 more)

### Community 154 - "本机文件存储"
Cohesion: 0.16
Nodes (15): directories_storage_dir(), FileStorage, read_ron(), roaming_appdata(), AsRef, Drop, HashMap, Into (+7 more)

### Community 155 - "界面构建参数"
Cohesion: 0.14
Nodes (9): Arc, Hash, Id, Into, Layout, Option, Rect, Self (+1 more)

### Community 156 - "文本编辑事件"
Cohesion: 0.17
Nodes (16): check_for_mutating_key_press(), events(), mask_if_password(), remove_ime_incompatible_events(), Align2, Arc, Event, FnMut (+8 more)

### Community 157 - "邮件本地操作"
Cohesion: 0.22
Nodes (4): Folder, App, Option, Result

### Community 158 - "响应体解码"
Cohesion: 0.19
Nodes (13): IoResult, Decoder, Decoder<R>, invalid_input1(), invalid_input2(), Option, R, Read (+5 more)

### Community 159 - "时间轴歌词解析"
Cohesion: 0.15
Nodes (22): cached_lyrics_preserve_timing_and_invalid_caches_are_ignored(), from_plain(), index_at_tracks_position(), is_credit(), is_native_script(), korean_and_japanese_romanization_is_recognized_without_flagging_real_lyrics(), Lyrics, lyrics_from_items() (+14 more)

### Community 160 - "网格控件布局"
Cohesion: 0.18
Nodes (9): Grid, Box, F, FnOnce, Hash, Id, Into, R (+1 more)

### Community 161 - "文本标签排版"
Cohesion: 0.16
Nodes (10): Label, Align, Arc, Galley, Into, Option, Pos2, Response (+2 more)

### Community 162 - "自动避让选择器"
Cohesion: 0.16
Nodes (13): Candidate, contains_app(), Picker, Cache, Option, Receiver, Rect, Result (+5 more)

### Community 163 - "毛玻璃对比度"
Cohesion: 0.17
Nodes (13): backdrop_estimate(), backdrop_estimate_leans_toward_bright_samples(), contrast(), dark_tint(), glass_tint(), glass_tint_keeps_white_text_readable_on_any_backdrop(), light_glass_keeps_dark_text_readable(), light_tint() (+5 more)

### Community 164 - "当前媒体选择"
Cohesion: 0.15
Nodes (17): active_music_wins_over_video_and_stale_paused_sessions(), choose_media(), choose_media_for_source(), kugou_repeated_samples_do_not_freeze_lyrics(), media_rank(), MediaKind, MediaMemory, metadata_gap_does_not_clear_lyrics_or_hide_loop_reset() (+9 more)

### Community 165 - "按类型索引数据"
Cohesion: 0.12
Nodes (9): IdTypeMap, D, Default, Error, Formatter, IntMap, Ok, Result (+1 more)

### Community 166 - "控件状态持久化"
Cohesion: 0.23
Nodes (5): from_ron_str(), FnOnce, Id, Option, T

### Community 167 - "网页文本输入代理"
Cohesion: 0.16
Nodes (12): Cell, HtmlInputElement, is_mobile_safari(), Drop, HtmlCanvasElement, JsValue, Option, Result (+4 more)

### Community 168 - "播放器进度定位"
Cohesion: 0.15
Nodes (17): DPI_AWARENESS_CONTEXT, IUIAutomationElement, DpiContext, matches_player(), point(), qq_bar(), Drop, HWND (+9 more)

### Community 169 - "控件标识生成"
Cohesion: 0.15
Nodes (10): NonZeroU64, Id, Debug, Formatter, From, Hash, IsEnabled, NodeId (+2 more)

### Community 170 - "HTTP Cookie 管理"
Cohesion: 0.14
Nodes (13): RwLockReadGuard, CookieStoreGuard, CookieStoreGuard<'a>, CookieTin, Cookie, CookieStore, Deref, I (+5 more)

### Community 171 - "无窗口演示录制"
Cohesion: 0.20
Nodes (16): Act, cursor_shapes(), ease(), gallery(), main(), Media, p(), Option (+8 more)

### Community 172 - "显示器与岛体定位"
Cohesion: 0.18
Nodes (14): Bounds, connected(), connected_displays_have_valid_bounds_and_distinct_ids(), display_selection_survives_reordering_disconnect_and_reconnect(), extreme_offsets_keep_the_expanded_panel_on_screen(), hide_for_fullscreen(), island_origin(), island_origin_offset() (+6 more)

### Community 173 - "侧边停靠绘制"
Cohesion: 0.18
Nodes (6): App, Pos2, Rect, Vec, side_clock(), SideItem

### Community 174 - "滑块交互绘制"
Cohesion: 0.19
Nodes (10): HandleShape, get(), GetSetValue, NumFormatter, NumParser, Pos2, Rangef, Rect (+2 more)

### Community 175 - "文本行与单词选择"
Cohesion: 0.21
Nodes (16): byte_index_from_char_index(), ccursor_next_line(), ccursor_next_word(), ccursor_previous_line(), ccursor_previous_word(), find_line_start(), is_linebreak(), is_word_char() (+8 more)

### Community 176 - "动画图像帧"
Cohesion: 0.14
Nodes (16): animated_image_frame_index(), are_animated_image_bytes(), decode_animated_image_uri(), encode_animated_image_uri(), FrameDurations, has_gif_magic_header(), has_webp_header(), is_animated_image_uri() (+8 more)

### Community 177 - "IMAP 元数据扩展"
Cohesion: 0.17
Nodes (10): check_admin(), check_entry_name(), check_path(), check_private_shared(), check_vendor_comment(), entry_name(), EntryParseStage, is_entry_component_char() (+2 more)

### Community 178 - "自定义 TLS 示例"
Cohesion: 0.16
Nodes (12): CustomTlsStream, main(), PassThrough, PassThroughError, Box, Error, Formatter, Option (+4 more)

### Community 179 - "项目说明与外观预览"
Cohesion: 0.11
Nodes (19): Daozai purple tile with black capsule mascot, Daozai mascot beside steaming coffee, Comparison of five island skins and two silhouettes, Notch and capsule silhouettes, Classic black, cream, light and dark glass, and floating cloud, Three screen-edge placements, Top, left and right docking, Adjustable island size from 80% to 140% (+11 more)

### Community 180 - "Windows 媒体会话查询"
Cohesion: 0.23
Nodes (16): GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionMediaProperties, HSTRING, IAsyncOperation, artwork_result(), live_client_seek(), media_result(), media_result_in() (+8 more)

### Community 181 - "指针命中测试"
Cohesion: 0.23
Nodes (18): buttons_on_window(), contains_circle(), find_closest(), find_closest_within(), hit_test(), hit_test_on_close(), HashMap, Id (+10 more)

### Community 182 - "进度条控件"
Cohesion: 0.20
Nodes (8): ProgressBar, ProgressBarText, Color32, CornerRadius, Into, Option, Response, Self

### Community 183 - "IMAP 集成测试"
Cohesion: 0.22
Nodes (17): SmtpTransport, append(), append_with_flags(), append_with_flags_and_date(), connect_insecure_then_secure(), connect_secure(), inbox(), inbox_uid() (+9 more)

### Community 184 - "软件光栅绘制"
Cohesion: 0.17
Nodes (11): Raster, ClippedPrimitive, HashMap, Option, Self, TextureId, TexturesDelta, Vec (+3 more)

### Community 185 - "日历同步调度"
Cohesion: 0.18
Nodes (14): automatic_sync_can_stop_resume_and_leave_manual_sync_available(), Cmd, month_grid(), month_grid_covers_leap_days_and_year_changes(), month_start(), multiple_calendars_sync_even_when_one_fails(), Duration, FnMut (+6 more)

### Community 186 - "ZCode 额度查询"
Cohesion: 0.25
Nodes (16): add(), coding(), fetch(), Login, path(), read(), Account, Config (+8 more)

### Community 187 - "文本编辑状态"
Cohesion: 0.18
Nodes (8): TextEditUndoer, Arc, Galley, Id, Mutex, Option, Self, TextEditState

### Community 188 - "尺寸调整把手"
Cohesion: 0.18
Nodes (12): paint_resize_corner(), paint_resize_corner_with_style(), Prepared, Align2, Color32, FnOnce, Id, Option (+4 more)

### Community 189 - "待办提醒选择器"
Cohesion: 0.22
Nodes (9): NaiveDateTime, TaskDraft, date_wheel_stops_at_today_and_ignores_date_overflow(), PickerAction, ReminderPicker, NaiveDate, Option, Self (+1 more)

### Community 190 - "弹簧动画插值"
Cohesion: 0.19
Nodes (9): clamp01(), critically_damped_never_overshoots(), ease_in_out(), ease_out_cubic(), frame_rate_independent(), remap(), Self, Spring (+1 more)

### Community 191 - "邮箱账户与服务"
Cohesion: 0.19
Nodes (13): Account, Cmd, credentials_are_bound_to_target_and_login(), exmail_live_capability_probe(), key(), load(), load_accounts(), qq_live_calendar_mail_probe() (+5 more)

### Community 192 - "邮件连接测试"
Cohesion: 0.19
Nodes (13): connect(), idle_without_push_returns_for_a_fallback_check(), login_error(), Prefixed<S>, probe(), read_line(), Duration, Error (+5 more)

### Community 193 - "额度账户设置"
Cohesion: 0.20
Nodes (3): App, claude_disclaimer(), Vec

### Community 194 - "网页 OpenGL 绘制"
Cohesion: 0.15
Nodes (10): Arc, ClippedPrimitive, ColorImage, Event, JsValue, Result, Self, TexturesDelta (+2 more)

### Community 195 - "逐帧发布缓存"
Cohesion: 0.15
Nodes (8): FramePublisher, FramePublisher<Key, Value>, Any, Default, HashMap, Option, Self, Value

### Community 196 - "折叠标题控件"
Cohesion: 0.22
Nodes (6): CollapsingHeader, Hash, IconPainter, Id, Into, Self

### Community 197 - "主题样式切换"
Cohesion: 0.18
Nodes (5): From, Option, Self, Theme, ThemePreference

### Community 198 - "用户数据比较"
Cohesion: 0.15
Nodes (13): Any, Arc, Deserialize, Eq, H, Hash, Option, PartialEq (+5 more)

### Community 199 - "模拟网络流"
Cohesion: 0.17
Nodes (7): MockStream, Default, Read, Result, Self, Vec, Write

### Community 200 - "隐私与使用指南"
Cohesion: 0.14
Nodes (16): Windows 凭据管理器与 DPAPI, 隐私与安全, 本机用户数据与文件路径, 邮件内容驻留进程内存, 本机音乐缓存与播放元数据, 按功能访问第三方服务, 只读使用原客户端登录文件, 安装包大小与 SHA-256 校验 (+8 more)

### Community 201 - "原生拖放窗口"
Cohesion: 0.16
Nodes (8): HWINEVENTHOOK, DragCapture, on_foreground(), Drop, Option, Pos2, Settings, Window

### Community 202 - "网页异常处理"
Cohesion: 0.19
Nodes (8): PanicHookInfo, PanicHandler, PanicHandlerInner, PanicSummary, Arc, Mutex, Option, Self

### Community 203 - "远程邮件操作反馈"
Cohesion: 0.17
Nodes (13): account(), failed_server_operation_keeps_the_message_and_reports_error(), identity(), local_trash_keeps_its_own_limit_when_inbox_also_has_recent_mail(), LocalFlags, Account, Color32, HashMap (+5 more)

### Community 204 - "托盘窗口消息"
Cohesion: 0.29
Nodes (13): activate_existing(), add_icon(), dump_menu(), remove_icon(), request_existing_window(), HWND, LPARAM, LRESULT (+5 more)

### Community 205 - "字体选择与排版"
Cohesion: 0.23
Nodes (9): TextFormat, TextWrapping, FontSelection, From, Align, Arc, Galley, LayoutJob (+1 more)

### Community 206 - "模态对话框"
Cohesion: 0.20
Nodes (9): Modal, ModalResponse, Color32, FnOnce, Frame, Id, Option, Response (+1 more)

### Community 207 - "输入事件过滤"
Cohesion: 0.17
Nodes (13): Event, EventFilter, ImeEvent, MouseWheelUnit, ActionRequest, ColorImage, Default, From (+5 more)

### Community 208 - "数值格式化"
Cohesion: 0.19
Nodes (10): NumberFormatter, Arc, Debug, Fn, Formatter, PartialEq, RangeInclusive, Result (+2 more)

### Community 209 - "类型标识映射"
Cohesion: 0.17
Nodes (9): hash(), PersistedMap, Arc, From, IsEnabled, Self, Vec, SerializedElement (+1 more)

### Community 210 - "连接归还读取"
Cohesion: 0.20
Nodes (7): NoopStream, PoolReturnRead<R>, Formatter, Read, Result, TcpStream, Write

### Community 211 - "定长缓存"
Cohesion: 0.17
Nodes (10): K, V, FixedCache, FixedCache<K, V>, Debug, Default, Formatter, Option (+2 more)

### Community 212 - "桌面背景采样"
Cohesion: 0.18
Nodes (10): Arc, AtomicBool, Drop, Mutex, Option, Self, Vec, sample() (+2 more)

### Community 213 - "拖放负载管理"
Cohesion: 0.21
Nodes (6): DragAndDrop, Any, Arc, Option, Send, Sync

### Community 215 - "托盘状态管理"
Cohesion: 0.18
Nodes (12): AtomicU32, Ctx, Arc, AtomicBool, Drop, Event, JoinHandle, Mutex (+4 more)

### Community 216 - "网页日志输出"
Cohesion: 0.20
Nodes (7): LevelFilter, Record, SetLoggerError, Result, Self, shorten_file_path(), WebLogger

### Community 217 - "HTTP 冒烟测试"
Cohesion: 0.24
Nodes (11): Log, get(), get_and_write(), get_many(), main(), Oops, Error, Formatter (+3 more)

### Community 218 - "表单与列表交互测试"
Cohesion: 0.14
Nodes (13): closing_app_notification_clears_its_bell_and_keeps_queued_messages(), focus_completion_offers_starting_the_break(), focus_durations_support_direct_input_and_keep_the_current_timer(), low_power_notice_dismisses_without_opening_the_mail_page(), mail_meeting_notice_opens_its_calendar_day(), messenger_notifications_keep_separate_badges_until_the_client_is_viewed(), todo_completed_reordering_does_not_complete_the_next_item_on_a_double_click(), todo_edit_choice_can_cancel_without_toggling_or_deleting_items() (+5 more)

### Community 219 - "面板布局收尾"
Cohesion: 0.21
Nodes (4): Prepared, Rect, Response, Shape

### Community 220 - "默认纹理加载器"
Cohesion: 0.18
Nodes (7): DefaultTextureLoader, Cow, HashMap, Mutex, TextureHandle, TextureLoadResult, TextureOptions

### Community 221 - "光标指针交互"
Cohesion: 0.22
Nodes (8): cursor_rect(), Cursor, From, Galley, Option, Rect, Response, TextCursorState

### Community 222 - "数值拖动边界"
Cohesion: 0.21
Nodes (10): clamp_value_to_range(), default_parser(), DragValue, get(), parse(), GetSetValue, NumFormatter, NumParser (+2 more)

### Community 223 - "图像按钮控件"
Cohesion: 0.29
Nodes (6): ImageButton<'a>, Color32, CornerRadius, Into, Rect, Self

### Community 224 - "网页画布尺寸监听"
Cohesion: 0.19
Nodes (10): Array, ResizeObserver, paint_and_schedule(), paint_if_needed(), ResizeObserverContext, Closure, Drop, FnMut (+2 more)

### Community 225 - "折叠区域绘制"
Cohesion: 0.40
Nodes (6): BodyRet, CollapsingResponse, Box, FnOnce, Option, R

### Community 226 - "依赖补丁与许可"
Cohesion: 0.17
Nodes (13): 依赖与第三方许可, eframe WGL 上下文复用与窗口缩放, egui 多缩放倍率字体纹理, IMAP IDLE 失败后轮询, IMAP 能力响应空格兼容, 锁定依赖公告检查, HTTPS 代理隧道请求格式, eframe 应用框架说明 (+5 more)

### Community 227 - "图标图像转换"
Cohesion: 0.23
Nodes (10): DynamicImage, ImageError, from_image(), from_png_bytes(), IconData, IconDataExt, IconData, Result (+2 more)

### Community 228 - "可序列化控件状态"
Cohesion: 0.21
Nodes (11): Serializer, Element, Any, Box, Clone, Debug, Deserialize, Send (+3 more)

### Community 229 - "岛体状态更新"
Cohesion: 0.15
Nodes (4): Phase, Pomo, Event, Into

### Community 230 - "后台轮询时钟"
Cohesion: 0.29
Nodes (9): backwards_clock_observation_does_not_panic_or_poll_early(), first_poll_is_ready_even_when_backdating_would_underflow(), PollClock, refresh_waits_until_interval_then_becomes_ready(), Duration, Instant, Option, Self (+1 more)

### Community 231 - "事件循环上下文"
Cohesion: 0.19
Nodes (10): EventLoopGuard, ActiveEventLoop, Drop, F, FnOnce, Option, R, Self (+2 more)

### Community 232 - "内存字节加载"
Cohesion: 0.17
Nodes (6): DefaultBytesLoader, BytesLoadResult, Cow, HashMap, Into, Mutex

### Community 233 - "超链接控件"
Cohesion: 0.26
Nodes (6): Hyperlink, Link, Into, Response, Self, ToString

### Community 234 - "分隔线控件"
Cohesion: 0.22
Nodes (5): Default, Option, Response, Self, Separator

### Community 235 - "加载指示器"
Cohesion: 0.23
Nodes (7): Color32, Into, Option, Rect, Response, Self, Spinner

### Community 236 - "文本缓冲区修改"
Cohesion: 0.18
Nodes (3): Cow<'_, str>, Range, &str

### Community 237 - "HTTP 请求工厂"
Cohesion: 0.32
Nodes (3): Agent, Request, Weak

### Community 238 - "启动诊断日志"
Cohesion: 0.33
Nodes (10): directory(), failure_creates_a_report_and_exposes_its_location(), install(), message(), report(), Path, PathBuf, Result (+2 more)

### Community 239 - "酷狗播放器控制"
Cohesion: 0.27
Nodes (9): endpoint(), player(), read_endpoint(), Option, Path, PathBuf, Value, seek() (+1 more)

### Community 240 - "邮箱连接端点"
Cohesion: 0.18
Nodes (3): CustomImap, Provider, Default

### Community 241 - "邮件事件与账户解析"
Cohesion: 0.23
Nodes (8): decode_accounts(), display_addr(), event_of(), ics_text(), meetings_of(), Option, Status, web_url()

### Community 242 - "应用标题图标"
Cohesion: 0.42
Nodes (9): AppIconStatus, AppTitleIconSetter, Arc, IconData, Option, Self, set_app_icon_windows(), set_title_and_icon() (+1 more)

### Community 243 - "字体布局与测量"
Cohesion: 0.30
Nodes (6): Arc, FnOnce, FontId, Fonts, Galley, LayoutJob

### Community 244 - "系统 TLS 连接"
Cohesion: 0.18
Nodes (9): default_tls_config(), native_tls::TlsConnector, native_tls::TlsStream<Box<dyn ReadWrite>>, Arc, Box, Error, Option, Result (+1 more)

### Community 245 - "媒体指令工作线程"
Cohesion: 0.25
Nodes (8): ControlRequest, ControlWorker, queued_seeks_for_the_same_player_run_only_the_latest(), resource_control_queue_stays_bounded_when_the_player_is_stuck(), resource_full_async_budget_prevents_starting_another_operation(), Fn, SyncSender, Transport

### Community 246 - "双侧布局控件"
Cohesion: 0.25
Nodes (6): RetL, RetR, FnOnce, Option, Self, Sides

### Community 247 - "日程编辑表单"
Cohesion: 0.24
Nodes (5): App, Cmd, Option, Pos2, Vec

### Community 248 - "番茄钟绘制"
Cohesion: 0.36
Nodes (10): draw(), fill_polygon(), liquid(), liquid_remains_bounded_when_full_nearly_empty_or_empty(), remaining_energy_area_decreases_with_time(), Color32, Vec, tea() (+2 more)

### Community 249 - "耗时测量"
Cohesion: 0.22
Nodes (4): Instant, Option, Self, Stopwatch

### Community 250 - "临时与持久状态"
Cohesion: 0.47
Nodes (7): test_mix(), test_mix_serialize(), test_one_id_two_types(), test_serialize_gc(), test_serialize_generations(), test_two_id_two_type(), test_two_id_x_two_types()

### Community 251 - "图形后端回退"
Cohesion: 0.31
Nodes (8): both_failures_are_preserved_in_the_report(), compatible(), forced_compatible(), opengl_20_failure_retries_without_opengl(), FnMut, Result, run(), successful_opengl_run_is_not_restarted()

### Community 252 - "输入帧初始化"
Cohesion: 0.29
Nodes (4): InputOptions, Default, Options, Self

### Community 253 - "图形变换与替换"
Cohesion: 0.31
Nodes (5): multiply_opacity(), I, R, Shape, tint_shape_towards()

### Community 254 - "文本光标绘制"
Cohesion: 0.29
Nodes (9): paint_cursor_end(), paint_text_cursor(), paint_text_selection(), RowVertexIndices, Arc, Galley, Option, Rect (+1 more)

### Community 255 - "复选框控件"
Cohesion: 0.27
Nodes (5): Checkbox, Checkbox<'a>, Into, Response, Self

### Community 256 - "滑块数值映射"
Cohesion: 0.40
Nodes (8): logarithmic_zero_cutoff(), normalized_from_value(), range_log10(), RangeInclusive, SliderClamping, SliderOrientation, SliderSpec, value_from_normalized()

### Community 257 - "ureq 发布流程"
Cohesion: 0.20
Nodes (10): ureq 2.12.1 依赖版本约束, ureq 版本记录, Rust 最低版本与 rustls 升级, ureq 构建测试矩阵, HTTP 功能与 TLS 测试矩阵, 更新发布记录, 发布前等待 CI, ureq 发布流程 (+2 more)

### Community 258 - "蓝牙设备图标"
Cohesion: 0.39
Nodes (9): Bluetooth rune for generic devices, Gamepad device silhouette, Headphone device silhouette, Keyboard device silhouette, Mouse device silhouette, Smartphone device silhouette, Speaker device silhouette, device_icon() (+1 more)

### Community 259 - "显示缩放探针"
Cohesion: 0.28
Nodes (7): main(), Probe, App, Frame, Instant, Result, Vec

### Community 260 - "第三方授权归属"
Cohesion: 0.22
Nodes (9): ai-usagebar 额度兼容参考, CodeBurn 额度字段参考, CodexBar Gemini 用量参考, 第三方版权与许可汇总, eframe 上游图形上下文修复, fly-island 消息提醒实现参考, grok-bot-usage 兼容参考, LobeHub AI 服务图标 (+1 more)

### Community 261 - "WebGL 上下文创建"
Cohesion: 0.47
Nodes (8): init_glow_context_from_canvas(), init_webgl1(), init_webgl2(), is_safari_and_webkit_gtk(), HtmlCanvasElement, Option, webgl1_requires_brightening(), WebGlRenderingContext

### Community 262 - "网页 wgpu 截图"
Cohesion: 0.28
Nodes (6): ClippedPrimitive, Event, Result, Self, TexturesDelta, Vec

### Community 263 - "调试文本输出"
Cohesion: 0.28
Nodes (6): Entry, print(), register(), Into, Vec, State

### Community 264 - "菜单栏内部状态"
Cohesion: 0.25
Nodes (4): BarState, Deref, DerefMut, Target

### Community 265 - "文本编辑输出"
Cohesion: 0.28
Nodes (7): Arc, Galley, Option, Pos2, Rect, Response, TextEditOutput

### Community 266 - "IMAP 邮箱名称"
Cohesion: 0.33
Nodes (5): Name, NameAttribute, Cow, Option, Vec

### Community 267 - "岛仔版本记录"
Cohesion: 0.25
Nodes (8): Claude 本地用量采集, 岛仔更新记录, 收信与阅读稳定性, 本机和远程邮件整理, 微信多账号提醒, 音乐与歌词兼容, 显示与毛玻璃性能, 安装与应用内更新

### Community 268 - "Windows 构建与测试"
Cohesion: 0.25
Nodes (8): 岛仔开发指南, 开发数据目录隔离, 合成数据与无窗口测试, Windows 检查工作流, 安装、升级与卸载验证, NSIS 发布构建与打包, Windows HTTP 代理请求回归测试, Windows 单线程测试

### Community 269 - "歌词内存回归"
Cohesion: 0.32
Nodes (3): GlobalAlloc, CountingAllocator, Layout

### Community 270 - "控件动画管理"
Cohesion: 0.36
Nodes (5): AnimationManager, BoolAnim, Id, IdMap, ValueAnim

### Community 271 - "操作系统识别"
Cohesion: 0.36
Nodes (3): OperatingSystem, Default, Self

### Community 273 - "HTTP 状态错误"
Cohesion: 0.32
Nodes (4): OrAnyStatus, Result<Response, Error>, Formatter, Result

### Community 275 - "窗口边缘响应"
Cohesion: 0.29
Nodes (4): BitAnd, Output, BitOrAssign, SideResponse

### Community 276 - "文本无障碍信息"
Cohesion: 0.33
Nodes (6): Role, Galley, Id, Option, TSTransform, update_accesskit_for_text_widget()

### Community 277 - "歌词缺失缓存"
Cohesion: 0.48
Nodes (7): cache_path(), confirmed_missing_lyrics_are_remembered_for_a_week_but_failures_are_not(), negative_path(), read_negative(), Path, PathBuf, write_negative()

### Community 278 - "本地 KRC 歌词"
Cohesion: 0.29
Nodes (7): is_soda(), krc_short_gaps_hold_previous_line_without_changing_timing(), native_fingerprint(), parse_krc(), read_native(), SystemTime, soda_krc_preserves_real_timing_and_instrumental_gaps()

### Community 279 - "网页本地存储"
Cohesion: 0.52
Nodes (6): load_memory(), local_storage(), local_storage_get(), local_storage_set(), Option, save_memory()

### Community 280 - "用户数据序列化"
Cohesion: 0.33
Nodes (5): D, Error, Ok, Result, S

### Community 281 - "指针交互快照"
Cohesion: 0.33
Nodes (5): interact(), InteractionSnapshot, Id, IdSet, Option

### Community 283 - "单选按钮控件"
Cohesion: 0.29
Nodes (4): RadioButton, Into, Response, Self

### Community 284 - "可选标签控件"
Cohesion: 0.29
Nodes (4): Into, Response, Self, SelectableLabel

### Community 285 - "STARTTLS 收件示例"
Cohesion: 0.52
Nodes (6): fetch_inbox_top(), main(), Box, Error, Option, Result

### Community 287 - "邮件阅读与验证码"
Cohesion: 0.33
Nodes (5): One-click verification code card, Email verification code and message reader, Email reader with masked verification code, 邮件阅读与验证码, App

### Community 288 - "图像纹理载入"
Cohesion: 0.33
Nodes (4): ImageData, TextureHandle, TextureLoadResult, TextureOptions

### Community 289 - "映射字体文件"
Cohesion: 0.33
Nodes (5): Mmap, MappedFont, File, Result, Self

### Community 290 - "网页运行器锁"
Cohesion: 0.47
Nodes (3): RefMut, ConcreteApp, Option

### Community 291 - "常驻组件排序"
Cohesion: 0.33
Nodes (4): valid_order(), crate::app::App, preview_layout(), Layout

### Community 292 - "本地凭据解密"
Cohesion: 0.47
Nodes (5): decrypt(), key(), Path, Result, Vec

### Community 293 - "解码错误消息"
Cohesion: 0.40
Nodes (4): DecoderError, Error, Formatter, Result

### Community 294 - "问题与安全反馈"
Cohesion: 0.40
Nodes (5): 问题反馈模板, 截图与日志脱敏, 可复现的问题报告, 私密安全问题反馈, GitHub 私密漏洞报告

### Community 295 - "托盘图标加载"
Cohesion: 0.70
Nodes (5): HICON, load_embedded_icon(), load_icon(), load_resource_icon(), Option

### Community 296 - "QQ 通知来源"
Cohesion: 0.60
Nodes (3): matches(), matches_in(), Path

### Community 297 - "界面缩放菜单"
Cohesion: 0.80
Nodes (4): zoom_in(), zoom_menu_buttons(), zoom_out(), zoom_with_keyboard()

### Community 298 - "IMAP 引号字符串"
Cohesion: 0.50
Nodes (3): quoted_string(), Cow, Result

### Community 300 - "聊天未读预览"
Cohesion: 0.50
Nodes (4): Separate messenger-account shortcuts, Compact unread badges and expanded messenger shortcuts, WeChat and QQ unread indicators, 微信和 QQ 未读提醒

### Community 301 - "会议提醒预览"
Cohesion: 0.50
Nodes (4): Upcoming meeting reminder card, Upcoming meeting countdown and view button, Meeting reminder atop a Windows desktop, Expanded meeting notification ten minutes before the event

### Community 302 - "零拷贝迭代"
Cohesion: 0.50
Nodes (3): IntoIter, &'a ZeroCopy<D>, IntoIterator

### Community 303 - "跟随系统配色"
Cohesion: 0.50
Nodes (3): color(), Color32, system_dark()

### Community 304 - "后台服务性能采样"
Cohesion: 0.67
Nodes (3): background_service_sample(), Memory, sample()

### Community 308 - "设备图标授权"
Cohesion: 1.00
Nodes (3): Creative Commons Attribution 4.0, 设备图标归属, 480 Design 的 Solar Icons

### Community 316 - "IMAP 版本记录"
Cohesion: 0.67
Nodes (3): IMAP 版本记录, 空集合 fetch 与 uid_fetch, STARTTLS 与超时示例

## Knowledge Gaps
- **85 isolated node(s):** `Region`, `App`, `crate::app::App`, `Memory`, `App` (+80 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **14 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `WorkBuddy 登录回调` to `日历配置与 CalDAV`, `微信窗口与未读提示`, `额度账户与配置`, `egui 上下文读取`, `岛体逐帧绘制`, `本地设置与主题`, `HTTP 连接与代理`, `岛体卡片绘制`, `原生 OpenGL 窗口`, `Exchange 与日历事件`, `Windows 系统通知`, `应用窗口初始化`, `原生 wgpu 视口`, `多视口配置`, `IMAP 会话命令`, `网易云播放数据`, `富文本与控件文字`, `HTTP 跳转与超时测试`, `岛体命令与状态`, `Grok 额度查询`, `IMAP 正文与能力`, `岛体功能页面`, `应用升级下载`, `IMAP 命令参数`, `剪贴板历史`, `资源监测与采样`, `额度页面与品牌图标`, `图层与浮动区域`, `平台输出与无障碍`, `日历页面与来源图标`, `键盘输入状态`, `IMAP IDLE 与时间线`, `请求发送与显示布局`, `HTTP 客户端配置`, `HTTP 请求执行`, `岛体设置控件`, `图像加载器`, `HTTP 首部解析`, `HTTP 响应读取`, `歌词来源与匹配`, `邮件解析与邀请`, `Gemini 额度读取`, `桌面外壳与命令`, `IMAP 零拷贝数据`, `岛体通知绘制`, `邮件操作执行`, `文本标签选择`, `界面堆栈元数据`, `WorkBuddy 额度查询`, `Claude 用量历史`, `功能截图与像素测试`, `滑块数值输入`, `收件箱后台监听`, `HTTP 响应流`, `网页应用运行器`, `图像控件构建`, `HTTP 连接池`, `HTTP 传输错误`, `控件自省工具`, `文本编辑缓冲区`, `图像来源与选项`, `IMAP 结果解析`, `邮件内容获取`, `IMAP 客户端示例`, `HTTP 命令行工具`, `浏览器界面适配`, `文件图标缓存`, `渲染轮次状态`, `蓝牙设备状态`, `邮件验证码隐私`, `Claude 桌面缓存`, `拖动数值控件`, `IMAP 解析错误`, `汽水音乐缓存`, `邮件验证码提取`, `HTTP 代理设置`, `网页触摸转换`, `DeepSeek 余额查询`, `在线歌词获取`, `网页事件监听`, `媒体封面缓存`, `托盘菜单构建`, `视口输入与命令`, `酷狗目录缓存`, `网页重绘请求`, `请求体与拖放数据`, `额度面板状态`, `酷狗曲目信息`, `本机文件存储`, `文本编辑事件`, `邮件本地操作`, `时间轴歌词解析`, `自动避让选择器`, `当前媒体选择`, `控件标识生成`, `侧边停靠绘制`, `滑块交互绘制`, `文本行与单词选择`, `动画图像帧`, `Windows 媒体会话查询`, `日历同步调度`, `ZCode 额度查询`, `待办提醒选择器`, `邮箱账户与服务`, `邮件连接测试`, `额度账户设置`, `网页 OpenGL 绘制`, `网页异常处理`, `远程邮件操作反馈`, `字体选择与排版`, `输入事件过滤`, `数值格式化`, `HTTP 冒烟测试`, `数值拖动边界`, `图标图像转换`, `岛体状态更新`, `超链接控件`, `文本缓冲区修改`, `HTTP 请求工厂`, `启动诊断日志`, `酷狗播放器控制`, `邮箱连接端点`, `邮件事件与账户解析`, `应用标题图标`, `字体布局与测量`, `媒体指令工作线程`, `日程编辑表单`, `WebGL 上下文创建`, `网页 wgpu 截图`, `调试文本输出`, `网页本地存储`, `STARTTLS 收件示例`, `图像纹理载入`, `本地凭据解密`?**
  _High betweenness centrality (0.517) - this node is a cross-community bridge._
- **Why does `Context` connect `egui 上下文读取` to `显示缩放探针`, `微信窗口与未读提示`, `WebGL 上下文创建`, `网页 wgpu 截图`, `桌宠表情与动效`, `岛体逐帧绘制`, `网页触摸转换`, `折叠面板状态`, `调试文本输出`, `控件交互响应`, `原生 OpenGL 窗口`, `操作系统识别`, `网页 wgpu 绘制`, `Windows 系统通知`, `视口输入与命令`, `应用窗口初始化`, `中文字体加载`, `原生 wgpu 视口`, `下拉框与提示`, `网页本地存储`, `侧栏与顶栏面板`, `请求体与拖放数据`, `菜单弹窗状态`, `文本无障碍信息`, `多视口配置`, `岛体命令与状态`, `滚动区域状态`, `邮件阅读与验证码`, `图像纹理载入`, `视口状态管理`, `自动避让选择器`, `浮动区域与图层`, `应用升级下载`, `网格控件布局`, `视口样式写入`, `界面缩放菜单`, `原生事件循环`, `额度页面与品牌图标`, `上下文默认配置`, `跟随系统配色`, `平台输出与无障碍`, `上下文调试输出`, `动画图像帧`, `Windows 媒体会话查询`, `岛体设置控件`, `系统媒体控制`, `尺寸调整把手`, `文本编辑状态`, `eframe 应用集成`, `岛体轮廓与停靠`, `额度账户设置`, `网页 OpenGL 绘制`, `桌面外壳与命令`, `折叠标题控件`, `主题样式切换`, `文本标签选择`, `原生窗口命中`, `嵌套控件响应`, `字体选择与排版`, `模态对话框`, `功能截图与像素测试`, `拖放负载管理`, `文本编辑事件`, `网页应用运行器`, `颜色选择控件`, `默认纹理加载器`, `缩放平移场景`, `图像来源与选项`, `窗口调整尺寸`, `内存字节加载`, `浏览器界面适配`, `网格布局状态`, `文件图标缓存`, `媒体指令工作线程`, `eframe 启动错误`, `egui 绘图上下文`?**
  _High betweenness centrality (0.213) - this node is a cross-community bridge._
- **Why does `Ui` connect `egui 界面构建` to `复选框控件`, `多点触摸状态`, `egui 上下文读取`, `折叠面板状态`, `岛体逐帧绘制`, `控件交互响应`, `岛体卡片绘制`, `嵌套控件响应`, `侧栏与顶栏面板`, `滚动条样式`, `视口输入与命令`, `下拉框与提示`, `文本编辑事件`, `菜单弹窗状态`, `指针交互快照`, `界面滚动定位`, `界面构建参数`, `单选按钮控件`, `岛体命令与状态`, `滚动区域状态`, `可选标签控件`, `网格控件布局`, `文本标签排版`, `自动避让选择器`, `常驻组件排序`, `应用升级下载`, `浮动区域与图层`, `控件命中与交互`, `界面缩放菜单`, `额度页面与品牌图标`, `滑块交互绘制`, `键盘输入状态`, `进度条控件`, `岛体设置控件`, `尺寸调整把手`, `额度账户设置`, `折叠标题控件`, `主题样式切换`, `岛体通知绘制`, `控件空间分配`, `文本标签选择`, `界面堆栈元数据`, `字体选择与排版`, `模态对话框`, `界面视觉样式`, `滑块数值输入`, `颜色选择控件`, `滚动动画设置`, `面板布局收尾`, `界面焦点配置`, `光标指针交互`, `控件自省工具`, `面板边框与边距`, `缩放平移场景`, `折叠区域绘制`, `图像来源与选项`, `窗口调整尺寸`, `超链接控件`, `分隔线控件`, `加载指示器`, `网格布局状态`, `egui 绘图上下文`, `按钮控件`, `双侧布局控件`, `日程编辑表单`, `拖动数值控件`, `输入帧初始化`, `文本光标绘制`, `控件样式与间距`?**
  _High betweenness centrality (0.150) - this node is a cross-community bridge._
- **What connects `Region`, `App`, `crate::app::App` to the rest of the system?**
  _85 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `界面状态回归测试` be split into smaller, more focused modules?**
  _Cohesion score 0.03408316291751875 - nodes in this community are weakly interconnected._
- **Should `日历配置与 CalDAV` be split into smaller, more focused modules?**
  _Cohesion score 0.047994000749906264 - nodes in this community are weakly interconnected._
- **Should `鼠标交互回归测试` be split into smaller, more focused modules?**
  _Cohesion score 0.044923629829290206 - nodes in this community are weakly interconnected._
## 查询基准

graphify benchmark 按节点数估计全量读取约 600,133 token。五个默认问题平均每次查询约 47,550 token，相当于该基线的 1/12.6。各问题的估计缩减倍数分别为身份验证 30.4、主入口 42.8、错误处理 13.0、数据层与 API 7.7、核心抽象 7.5。该基线使用工具的节点数估算，与上面的实际扫描词数采用不同口径。
