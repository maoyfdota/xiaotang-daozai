# Graph Report - /Users/maoyufeng/RustStudy/xiaotang-daozai/src  (2026-11-19)

## 分析范围与核验说明

本次分析覆盖 `src/` 的 118 个文件，扫描词数为 168,553。图谱对应分支 `my_feature_1119`，提交 `08250c583dfda4fe3643af9e27cc93221b4170e9`。117 个 Rust 文件产生节点，`calendar/tls_test_certificates.json` 为测试证书数据，AST 提取未产生节点。

交互图使用社区聚合视图，包含 139 个社区节点和 418 条跨社区连线。JSON 保存 5,371 个节点和 14,865 条关系。社区是算法聚类结果，个别社区混合了不同模块，应结合源码判断职责。

图谱默认采用无向结构。原始关系中有 406 条悬空端点关系、2 条自环和 388 条完全重复关系，缺失端点字段为 0。有向同端点折叠数量为 1,208，无向口径为 1,257。图谱适合辅助定位，依赖方向和具体调用需要结合源码核验。

自动发现的五条 Surprising Connections 中，微信测试调用共享进程树函数已经源码核验。两条 `inside()` 和两条 `p()` 跨文件关系来自局部闭包名称与其他文件函数重名，均为错误推断。图中保留工具原始 INFERRED 标记以便审计，阅读这四条关系时应参考下方核验结论。

AST 提取调用没有产生独立 LLM API 用量，工具统计为 0 输入和 0 输出 token。社区命名与本报告说明由当前会话完成，其会话用量未计入该统计。

基准使用扫描词数估算全文上下文为 224,737 token，五个工具样例问题平均检索上下文为 14,395 token，估算缩减为 15.6 倍。该数字是工具的近似上下文统计。

## 蓝牙功能核查

| 能力 | 当前实现 | 源码位置 |
|---|---|---|
| 已连接设备监听 | 经典蓝牙和 BLE，每 2 秒轮询一次 | `bt.rs` 第 81、102 行 |
| 设备类型 | 耳机、音箱、键鼠、手柄、手机等 | `bt.rs` 第 14、16 行 |
| 电量读取 | 可选电量，连接集合变化时刷新，稳定时最多每 60 秒读取 | `bt.rs` 第 59、140 行 |
| 连接与断开提醒 | 首次扫描作为基线，后续按设备 ID 比较变化 | `app.rs` 第 1491 行 |
| 通知展示 | 设备名称、类型图标、连接状态和可用电量 | `extras.rs` 第 269、443 行 |
| 设置开关 | “系统提示”中的“蓝牙设备”，关闭后停止监听 | `extras.rs` 第 1862 行，`main.rs` 第 465 行 |
| 主动管理 | 在 `src/` 中尚未发现设备配对、主动连接、主动断开、忘记设备和蓝牙总开关入口 | 已核对蓝牙模块、调用方和 Windows 相关 API 使用 |

服务通过 `bt::spawn()` 创建后台线程，`query()` 读取已连接设备，`main.rs` 接收设备列表并放入 `Env`，`App::sync_device()` 比较设备变化，`bluetooth_alert()` 将提醒送入通知队列。蓝牙音频设备连接时会清除待显示或当前显示的系统音量提示。

Windows 条件编译代码负责系统蓝牙枚举和电量读取。此次结论来自静态源码检查，未在 Windows 真机上执行设备操作。

## Corpus Check
- 118 files · ~168,553 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 5371 nodes · 14865 edges · 139 communities (137 shown, 2 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 249 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `08250c58`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- 界面状态回归测试
- 鼠标交互回归测试
- 额度账户与解析
- 天气数据与状态
- Exchange 日历与波点音乐
- 中转服务额度适配
- Kiro 额度与浏览器登录
- Windows 系统状态
- Grok 额度与邮件操作
- 桌宠表情与皮肤
- 微信窗口与托盘
- 岛体轮廓与显示布局
- 额度设置与账户界面
- 托盘菜单与登录回调
- 本地设置与持久化
- 原生窗口与命中检测
- 分层窗口与文件图标
- 日历页面与本地日程
- 应用更新与下载
- 桌面外壳与轮询时钟
- 天气页面与图标
- API 额度账户连接
- 邮件管理与隐私
- 验证码与文字方向
- 系统媒体控制
- 剪贴板历史
- 常驻栏布局与动画
- 网易云播放数据
- 后台资源调度
- Gemini 额度查询
- 酷我播放与进度
- 天气来源设置
- 应用状态与页面导航
- CalDAV 地址与解析
- 窗口拖动测试框架
- 汽水音乐缓存
- 输入语言与快捷键
- 聊天窗口唤起
- WorkBuddy 额度查询
- 卡片绘图坐标与文本
- 聊天来源与账号未读
- 设置页面与控件
- 聊天客户端窗口定位
- 岛体功能页面
- 豆包额度连接
- 邮件正文格式化
- Claude 本地用量
- QQ 账号与窗口
- 媒体封面与查询缓存
- 流体皮肤绘制
- 邮箱后台监听
- 绘图基础与设备图标
- 聊天进程与通知账号
- 图形后端与诊断工具
- ZCode 额度查询
- 岛体逐帧绘制
- 窗口模糊与合成
- 提醒与任务编辑
- 邮件日历歌词状态
- 邮件内容获取
- 豆包浏览器登录
- 歌词解析与查询
- MiniMax 与 GLM 额度
- DeepSeek 余额查询
- 岛体初始化与命令
- 邮件账户与邀请解析
- 天气界面测试与主题
- 蓝牙设备状态
- 网易云时间线
- 当前媒体与曲目间隔
- 网易云封面与详情缓存
- 日历兼容与事件解析
- Claude 桌面缓存
- 通知绘制与健康提醒
- 酷狗曲目信息
- 歌词来源与匹配
- CalDAV 请求与错误
- 功能截图与界面回归
- 设置窗口位置
- 豆包额度接口测试
- 额度页面交互测试
- 启动诊断日志
- 播放器独立音量
- 酷狗目录缓存
- 和风天气客户端
- 和风天气数据解析
- 日历配置与同步
- 媒体控制工作线程
- 邮件事件与解析状态
- 自动避让选择器
- 字体发现与加载
- 毛玻璃配色与对比度
- 健康提醒交互测试
- 应用计时与日历事件
- 日历平台与诊断信息
- 演示录制脚本
- 日历同步调度
- 媒体封面搜索
- 播放器进度定位
- 歌词缺失缓存
- 软件光栅绘制
- IMAP 连接与超时
- 邮箱账户与服务
- 待办提醒选择器
- 桌面背景采样
- 日历发现回归测试
- 在线歌词查询测试
- 歌词来源回退
- 邮箱服务端配置
- 酷狗曲目缓存
- 歌词切换回归测试
- 侧边停靠几何
- 日历 TLS 传输测试
- 酷狗播放器控制
- 图形渲染回退
- 豆包额度界面测试
- 音乐诊断日志
- 和风天气刷新调度
- 设置数值与健康时段
- 番茄钟绘制
- 播放器进程识别
- 和风天气配置与用量
- 未读堆叠与玻璃绘制
- 天气城市设置
- 系统通知来源
- 邮件操作交互测试
- 本机凭据解密
- 侧边常驻栏绘制
- 子进程启动
- 本地歌词指纹
- 多语言字形支持
- 映射字体文件
- Apple Music 元数据
- 歌词缺失缓存开关
- 控件拖动排序测试

## God Nodes (most connected - your core abstractions)
1. `App` - 157 edges
2. `Card` - 137 edges
3. `Lyrics` - 123 edges
4. `Messenger` - 69 edges
5. `Quota` - 64 edges
6. `NowPlaying` - 61 edges
7. `App` - 46 edges
8. `Env` - 42 edges
9. `Shell` - 39 edges
10. `Item` - 38 edges

## Surprising Connections

- 微信进程树测试调用 `messenger/windows.rs` 的 `parent_processes()`，复用了聊天客户端公共模块。图中标记为 INFERRED，已核对 `wechat.rs` 第 10、759 行及 `messenger/windows.rs` 第 146 行。
- 图中两条模糊轮廓测试到 `shape.rs::inside()` 的关系为错误推断。测试实际调用本地 `inside` 闭包，可在 `app.rs` 第 336、369 行核对。
- 图中 `tomato_outline()`、`chair()` 到录制工具 `p()` 的关系为错误推断。实际调用的是各自函数内的 `p` 闭包，可在 `focus_bar.rs` 第 18 行、`paint.rs` 第 556 行核对。

## Import Cycles
- 1-file cycle: `input.rs -> input.rs`
- 1-file cycle: `system.rs -> system.rs`
- 2-file cycle: `pet.rs -> pet_expressions.rs -> pet.rs`

## Communities (139 total, 2 thin omitted)

### Community 0 - "界面状态回归测试"
Cohesion: 0.03
Nodes (202): about_support_preview(), app_notifications_show_temporary_bell_in_minimal_mode(), avoidance_icon_area_stays_one_row_and_scrolls(), background_services_follow_the_module_switch(), battery_module_can_disable_only_the_power_warning(), bluetooth_audio_connection_takes_precedence_over_volume_notice(), bluetooth_badge_follows_connect_and_disconnect(), bluetooth_connection_soon_after_initial_scan_is_not_suppressed() (+194 more)

### Community 1 - "鼠标交互回归测试"
Cohesion: 0.03
Nodes (132): alphabetic_mail_codes_reach_the_card_without_changing_case(), an_open_page_and_lyrics_can_coexist_with_notifications_and_the_bell(), arrows_and_dots_switch_pages(), battery_module_hides_power_in_collapsed_and_expanded_headers(), blank_todo_draft_closes_when_window_focus_is_lost(), bluetooth_switch_clears_notices_and_restarts_with_fresh_baseline(), browser_minimal_closes_the_open_island_but_still_allows_deliberate_hover(), calendar_dates_are_readable_and_source_labels_do_not_take_a_second_line() (+124 more)

### Community 2 - "额度账户与解析"
Cohesion: 0.05
Nodes (75): Clone, PendingAccounts, Accent, Account, account_credentials(), Amount, cadence(), claude_local_config_ignores_retired_reset_fields() (+67 more)

### Community 3 - "天气数据与状态"
Cohesion: 0.05
Nodes (77): base_name(), cache_matches(), city_candidates(), city_queries(), CitySearch, clock(), clock_formats_local_time(), Day (+69 more)

### Community 4 - "Exchange 日历与波点音乐"
Cohesion: 0.06
Nodes (86): control(), cover_url(), data_root(), Database, decode_lyrics(), discovery_due(), fetch_cover(), history_is_matched_to_the_engine_and_cover_hosts_are_checked() (+78 more)

### Community 5 - "中转服务额度适配"
Cohesion: 0.05
Nodes (87): Amount, auto_billing_uses_at_most_five_requests_and_does_not_invent_scope_or_currency(), auto_budget_and_cancellation_stop_further_requests_and_reject_late_success(), auto_does_not_accept_conflicting_currency_or_missing_billing_usage(), auto_falls_back_to_sub2api_before_compatible_billing(), auto_http_auth_rate_limit_redirect_and_server_errors_do_not_probe_other_routes(), auto_http_flow_preserves_origin_prefix_and_bearer_with_bounded_clients(), auto_prefers_key_quota_and_keeps_account_quota_separate() (+79 more)

### Community 6 - "Kiro 额度与浏览器登录"
Cohesion: 0.06
Nodes (83): check(), cookie_header(), Endpoint, gemini_page(), login(), login_at(), native_chrome_cdp_routes_large_replies_and_only_disconnects(), native_chrome_skips_usage_tab_without_an_identity() (+75 more)

### Community 7 - "Windows 系统状态"
Cohesion: 0.05
Nodes (80): AtomicU8, Cell, IAudioEndpointVolume, IDataObject, IMMDevice, NotificationFilter, audio(), audio_device() (+72 more)

### Community 8 - "Grok 额度与邮件操作"
Cohesion: 0.06
Nodes (90): account_slots_survive_string_wrapping_and_do_not_follow_the_active_slot(), accounts(), challenge(), checksum(), claims(), Connected, credential(), decrypt() (+82 more)

### Community 9 - "桌宠表情与皮肤"
Cohesion: 0.05
Nodes (69): approach(), LayerId, Expression, Expressions, expressions_follow_real_scenarios_and_system_animation_setting(), focus_music_sleep_and_warning_keep_priority(), hidden_feedback_pauses_and_hover_has_a_cooldown(), only_the_playing_sheet_is_uploaded_and_light_skins_keep_dark_eyes() (+61 more)

### Community 10 - "微信窗口与托盘"
Cohesion: 0.06
Nodes (70): visible_main_window_of(), matches(), window_pid(), account_main_window(), account_pid(), all_taskbar_clients_first_flash_after_minimizing_a_viewed_account_is_not_swallowed(), Burst, button_sized() (+62 more)

### Community 11 - "岛体轮廓与显示布局"
Cohesion: 0.05
Nodes (73): ComputerMut, any_offset_and_scale_keeps_the_panel_on_screen(), avoiding_the_top_bar_starts_below_it_and_keeps_other_edges(), Bounds, centered_island_has_the_same_room_on_both_sides(), connected(), connected_displays_have_valid_bounds_and_distinct_ids(), Display (+65 more)

### Community 12 - "额度设置与账户界面"
Cohesion: 0.06
Nodes (56): Connect, Connected, Local, Login, accent_color(), accent_colors(), api_details_include_third_window_tools_and_resets(), App (+48 more)

### Community 13 - "托盘菜单与登录回调"
Cohesion: 0.05
Nodes (67): HICON, HMENU, PCWSTR, executable_description(), about(), activate_existing(), add_icon(), Ctx (+59 more)

### Community 14 - "本地设置与持久化"
Cohesion: 0.06
Nodes (36): atomic_write(), atomic_write_replaces_existing_settings(), config_dir(), env_dir(), human_size(), island_appearance_values_are_normalized(), island_size_value(), IslandShape (+28 more)

### Community 15 - "原生窗口与命中检测"
Cohesion: 0.06
Nodes (52): Backend, HWINEVENTHOOK, MSLLHOOKSTRUCT, activation_keeps_focus_messages_but_suppresses_nonclient_paint(), apply_style(), compositor_blur_allowed(), DragCapture, editing_mode_does_not_recalculate_the_native_window_frame() (+44 more)

### Community 16 - "分层窗口与文件图标"
Cohesion: 0.05
Nodes (59): HBITMAP, HDC, HGDIOBJ, Cache, file_icon(), folders_use_system_icons_and_images_keep_content_thumbnails(), load_preview(), Loaded (+51 more)

### Community 17 - "日历页面与本地日程"
Cohesion: 0.07
Nodes (47): App, fallback_ics_uid(), Cmd, Option, Pos2, String, Ui, Vec (+39 more)

### Community 18 - "应用更新与下载"
Cohesion: 0.06
Nodes (57): automatic_check_reschedules_success_and_silent_failure(), automatic_check_waits_for_startup_and_idle_state(), automatic_notice_is_once_per_version_and_manual_check_can_reopen_it(), background_result_and_disconnect_allow_retry(), cancellation_before_install_handoff_keeps_the_app_open(), check(), check_error(), clip_update_window() (+49 more)

### Community 19 - "桌面外壳与轮询时钟"
Cohesion: 0.06
Nodes (52): ExitCode, background_repaint_keeps_the_requested_frame_interval(), cursor_logical(), default_apps_urls_and_other_errors_never_open_the_file_picker(), error(), local_now(), main(), matching_without_copy_agrees_with_the_copied_signature() (+44 more)

### Community 20 - "天气页面与图标"
Cohesion: 0.08
Nodes (48): Kind, ambient_clock(), App, bolt(), chevron_down(), city_row(), cloud(), cloud_shape() (+40 more)

### Community 21 - "API 额度账户连接"
Cohesion: 0.08
Nodes (50): cleanup_keeps_failed_refs_and_never_deletes_active_keys(), cleanup_pending(), Config, config_has_only_opaque_reference_and_endpoint(), Connect, Connected, credential(), custom_credentials() (+42 more)

### Community 22 - "邮件管理与隐私"
Cohesion: 0.07
Nodes (35): Folder, IntoIterator, Item, Action, account(), App, failed_server_operation_keeps_the_message_and_reports_error(), identity() (+27 more)

### Community 23 - "验证码与文字方向"
Cohesion: 0.06
Nodes (52): after(), alphabetic_code_at(), alphabetic_codes_reject_regular_words_and_non_authentication_fields(), around(), before(), candidate(), clock(), Code (+44 more)

### Community 24 - "系统媒体控制"
Cohesion: 0.06
Nodes (47): CO_MTA_USAGE_COOKIE, GlobalSystemMediaTransportControlsSession, Ordering, artwork_decodes_png_jpeg_webp_and_rejects_invalid_bytes(), browser_video_authors_are_not_enough_to_identify_a_song(), control(), decode_artwork(), idle_interval() (+39 more)

### Community 25 - "剪贴板历史"
Cohesion: 0.08
Nodes (37): Clip, dropping_the_watcher_ends_the_listener_thread(), excluded(), History, history_copy_preserves_order_but_external_copy_promotes(), history_dedupes_caps_and_keeps_pins(), pinned_round_trip_and_preview(), preview() (+29 more)

### Community 26 - "常驻栏布局与动画"
Cohesion: 0.07
Nodes (31): clamp01(), critically_damped_never_overshoots(), ease_in_out(), ease_out_cubic(), frame_rate_independent(), remap(), Self, Spring (+23 more)

### Community 27 - "网易云播放数据"
Cohesion: 0.10
Nodes (41): decode(), detail_lookup_runs_in_background_retries_then_stops(), encode(), event(), historical_date_handles_new_year_without_using_log_counter(), historical_millis(), historical_replay_restores_mid_song_and_then_uses_local_elapsed_time(), incomplete_track() (+33 more)

### Community 28 - "后台资源调度"
Cohesion: 0.08
Nodes (34): append(), append_log(), append_rotating(), blocked_jobs_have_a_process_wide_bound_and_release_their_slots(), cancellation_does_not_release_a_still_running_request(), diagnostic_history_rotates_and_rejects_oversized_records(), feature_logs_rotate_and_cap_each_line(), Guard (+26 more)

### Community 29 - "Gemini 额度查询"
Cohesion: 0.12
Nodes (45): address(), Connect, Connected, cookie(), decode(), email(), fetch(), forget() (+37 more)

### Community 30 - "酷我播放与进度"
Cohesion: 0.11
Nodes (37): click(), click_button(), control(), control_bounds(), cover_url(), decode(), DpiContext, fetch_cover() (+29 more)

### Community 31 - "天气来源设置"
Cohesion: 0.08
Nodes (24): Location, App, provider_option(), qweather_city_autocomplete_debounces_deduplicates_and_discards_old_results(), qweather_step(), Arc, AtomicBool, Config (+16 more)

### Community 32 - "应用状态与页面导航"
Cohesion: 0.06
Nodes (15): App, PositionDrag, Arc, Cache, ColorImage, Galley, JoinHandle, Mutex (+7 more)

### Community 33 - "CalDAV 地址与解析"
Cohesion: 0.08
Nodes (31): basic_auth(), calendar_collections(), calendar_data(), calendar_home(), calendar_limit_counts_only_unique_events_in_range(), clean(), dedupe_key(), ensure_slash() (+23 more)

### Community 34 - "窗口拖动测试框架"
Cohesion: 0.07
Nodes (46): ClippedShape, DroppedFile, HoveredFile, adding_a_todo_saves_content_then_offers_an_optional_reminder(), automatic_minimal_hides_unread_hit_area_until_reopened(), clock_residency_only_hides_the_collapsed_clock(), clock_seconds_keep_resident_spacing_and_digit_width(), H (+38 more)

### Community 35 - "汽水音乐缓存"
Cohesion: 0.09
Nodes (34): decode(), Decoder, Decoder<'a>, entries(), fingerprint(), fixture(), large_cache_is_mapped_and_read_with_reasons_for_misses(), le() (+26 more)

### Community 36 - "输入语言与快捷键"
Cohesion: 0.10
Nodes (32): active_profile(), actual_switch_keys_require_a_matching_state_change_and_are_used_once(), changing_app_windows_synchronizes_their_modes_without_notifying(), changing_focused_controls_does_not_reuse_another_controls_mode(), chinese(), ctrl_shift_detects_tsf_profile_changes_even_when_the_layout_is_unchanged(), ctrl_shift_detects_two_layouts_with_the_same_language_id(), delayed_input_mode_restoration_after_activation_is_silent() (+24 more)

### Community 37 - "聊天窗口唤起"
Cohesion: 0.11
Nodes (33): OpenError, String, account_tray_pid(), ancestors(), cached_targets_keep_client_namespaces_and_reject_mismatched_open_requests(), check_target(), foreground_client(), NativeAdapter (+25 more)

### Community 38 - "WorkBuddy 额度查询"
Cohesion: 0.12
Nodes (39): api(), Auth, Connect, Connected, Credits, Failure, fetch(), forget() (+31 more)

### Community 39 - "卡片绘图坐标与文本"
Cohesion: 0.10
Nodes (21): Card<'a>, chevron(), clock(), clock_width(), cover_gradient_keeps_all_four_corners_clear(), gear(), H, long_email_links_do_not_create_hundreds_of_layouts() (+13 more)

### Community 40 - "聊天来源与账号未读"
Cohesion: 0.09
Nodes (23): messenger_icon(), account_history_rejects_foreign_client_samples_and_keeps_notification_namespaces_separate(), AccountHistory, Adapter, Capture, Messenger, qq_icon(), registered_adapters_have_distinct_identities_and_matching_backends() (+15 more)

### Community 41 - "设置页面与控件"
Cohesion: 0.11
Nodes (36): island_size_drag_commits_only_on_release(), island_size_slider(), number_stepper(), number_stepper_clicks_follow_steps_and_limits(), number_stepper_fits_with_settings_button_padding(), release_support_codes(), ColorImage, Context (+28 more)

### Community 42 - "聊天客户端窗口定位"
Cohesion: 0.10
Nodes (40): candidates(), feishu_frame(), feishu_main_frame(), feishu_owner(), feishu_title(), flash_target(), has_dingtalk_frame(), live_feishu_tray_window_is_shown_by_feishu() (+32 more)

### Community 43 - "岛体功能页面"
Cohesion: 0.14
Nodes (16): lerp(), App, cover_colors(), mmss(), Cmd, Color32, Option, Path (+8 more)

### Community 44 - "豆包额度连接"
Cohesion: 0.14
Nodes (34): amount(), Connect, Connected, ConnectionSlot, cookie(), delete_slot(), fetch(), forget() (+26 more)

### Community 45 - "邮件正文格式化"
Cohesion: 0.10
Nodes (39): clean(), data_table_keeps_columns_and_bolds_its_header(), decode(), decoding_many_unterminated_entities_is_bounded(), entities_are_decoded(), from_html(), from_text(), html_headings_lists_and_quotes_keep_their_kind() (+31 more)

### Community 46 - "Claude 本地用量"
Cohesion: 0.17
Nodes (39): account(), bound_desktop_login_does_not_adopt_a_peer_record_when_switching_back(), code_fixture(), code_identity(), code_path(), code_slot(), desktop_identity(), desktop_record() (+31 more)

### Community 47 - "QQ 账号与窗口"
Cohesion: 0.10
Nodes (39): hidden_helpers_are_never_shown(), accounts(), live_probe(), open(), paths(), qq_multiple_main_windows_in_one_process_are_not_a_unique_account(), qq_single_tray_account_ignores_its_auxiliary_main_windows(), qq_wake_records_why_neither_window_nor_tray_could_be_used() (+31 more)

### Community 48 - "媒体封面与查询缓存"
Cohesion: 0.14
Nodes (34): ArtworkRequest, GlobalSystemMediaTransportControlsSessionManager, GlobalSystemMediaTransportControlsSessionMediaProperties, HSTRING, IAsyncOperation, artwork_cache_replaces_a_temporary_cover_on_the_same_song(), artwork_cache_retries_late_images_and_clears_on_song_change(), artwork_result() (+26 more)

### Community 49 - "流体皮肤绘制"
Cohesion: 0.13
Nodes (23): base_is_neutral_gray_not_a_neon_wash(), bloom_is_gone_so_animating_stops_after_the_fade(), chase_light_follows_the_head_and_is_soft(), collapsed_is_translucent_and_expanded_fades_toward_the_bottom(), color_returns_to_neutral_when_music_stops(), compose_alpha_stays_within_bounds(), cursor_glow_fades_in_and_out(), cursor_glow_is_smooth_across_neighbouring_samples() (+15 more)

### Community 50 - "邮箱后台监听"
Cohesion: 0.12
Nodes (37): AtomicU64, bursts_do_not_skip_messages_between_batches(), collect(), cooldown_wait_can_be_cancelled(), delete(), Event, fetch_new(), inbox() (+29 more)

### Community 51 - "绘图基础与设备图标"
Cohesion: 0.16
Nodes (30): alpha(), battery(), bell(), chair(), check(), checklist(), clipboard_glyph(), close() (+22 more)

### Community 52 - "聊天进程与通知账号"
Cohesion: 0.11
Nodes (27): AsRef, client_processes(), ClientProcess, confirming_a_view_only_rearms_that_clients_account(), exited_accounts_report_their_status_even_when_other_accounts_remain(), exited_messenger_accounts_are_removed_individually(), messenger_reminders_survive_hidden_or_destroyed_windows(), opening_clients_share_one_worker_slot_until_the_worker_finishes() (+19 more)

### Community 53 - "图形后端与诊断工具"
Cohesion: 0.10
Nodes (31): Backends, DeviceDescriptor, dirs_desktop(), launch(), requested(), Iterator, Option, Path (+23 more)

### Community 54 - "ZCode 额度查询"
Cohesion: 0.15
Nodes (33): add(), china_and_legacy_logins_query_only_their_own_entitlements(), china_login_is_accepted_without_merging_overseas_accounts(), coding(), fetch(), Fixture, hash(), Login (+25 more)

### Community 55 - "岛体逐帧绘制"
Cohesion: 0.15
Nodes (5): smoothstep(), Env, lyric_key_matches(), Context, Ui

### Community 56 - "窗口模糊与合成"
Cohesion: 0.10
Nodes (29): BlurPiece, covered(), animation_frames_do_not_reset_the_backdrop_region_every_frame(), Backdrop, backdrop_initialization_respects_system_support_and_starts_hidden(), backdrop_only_covers_the_blur_pieces_so_other_apps_stay_clickable(), backdrop_proc(), clip_rects() (+21 more)

### Community 57 - "提醒与任务编辑"
Cohesion: 0.10
Nodes (8): AlertSource, App, bluetooth_notice_content(), parse_due(), Option, String, settings_tab_module(), TaskDraft

### Community 58 - "邮件日历歌词状态"
Cohesion: 0.10
Nodes (22): CalendarFeed, CalendarForm, CalendarState, lyric_key_matches_the_formatted_key_without_allocating(), LyricsJob, LyricSong, LyricsState, MailForm (+14 more)

### Community 59 - "邮件内容获取"
Cohesion: 0.10
Nodes (28): BodyStructure, Fetch, account(), item(), large_binary_attachment_is_never_requested_and_full_text_is_kept(), legacy(), literal(), live_segment_fetch_matches_legacy_for_recent_mail() (+20 more)

### Community 60 - "豆包浏览器登录"
Cohesion: 0.12
Nodes (28): Browser, cdp(), check_cancel(), decrypt(), endpoint(), find_browser(), header(), local() (+20 more)

### Community 61 - "歌词解析与查询"
Cohesion: 0.09
Nodes (26): fetch(), http_search_and_network_errors_are_distinct_from_missing_lyrics(), index_at_tracks_position(), is_credit(), is_native_script(), korean_and_japanese_romanization_is_recognized_without_flagging_real_lyrics(), krc_short_gaps_hold_previous_line_without_changing_timing(), live_lead_artist_only_finds_collaboration_lyrics() (+18 more)

### Community 62 - "MiniMax 与 GLM 额度"
Cohesion: 0.16
Nodes (28): glm_business_errors_are_specific_without_echoing_private_messages(), glm_error(), glm_http_and_business_errors_preserve_their_meaning_and_retry_delay(), glm_pr46_response_keeps_plan_periods_and_rejects_failed_payloads(), glm_used_percentage_windows_and_tools_are_separate(), iso(), millis(), minimax_legacy_remaining_and_epoch_milliseconds() (+20 more)

### Community 63 - "DeepSeek 余额查询"
Cohesion: 0.13
Nodes (25): Balance, Connect, Connected, credential(), deepseek_live_read_only_diagnostic(), deepseek_requires_valid_amounts_and_preserves_currency(), fetch(), forget() (+17 more)

### Community 64 - "岛体初始化与命令"
Cohesion: 0.15
Nodes (20): blur_follows_the_island_outline_not_its_bounding_box(), blur_pieces(), Cmd, collapsing_pill_blur_radius_fits_the_physical_bounds(), FrameOut, GlassShape, NotificationSound, Options (+12 more)

### Community 65 - "邮件账户与邀请解析"
Cohesion: 0.10
Nodes (25): accounts_path(), accounts_roundtrip_without_secrets(), all_day_calendar_event_has_exclusive_end(), calendar_part_is_collected_from_mime(), credentials_are_bound_to_target_and_login(), custom_config_roundtrips_without_a_secret(), encode_accounts(), full_body_keeps_paragraphs_and_content_beyond_preview_and_old_fetch_limit() (+17 more)

### Community 66 - "天气界面测试与主题"
Cohesion: 0.12
Nodes (26): FullOutput, color(), Color32, Context, Self, SettingsTheme, system_dark(), ViewportId (+18 more)

### Community 67 - "蓝牙设备状态"
Cohesion: 0.14
Nodes (25): bluetooth_connection_change_refreshes_immediately_and_failed_scans_back_off(), byte_prop(), Cache, Device, device_kind(), DeviceKind, dropping_service_signals_stop_even_without_successful_scans(), idle_bluetooth_does_not_rescan_all_hardware_every_two_seconds() (+17 more)

### Community 68 - "网易云时间线"
Cohesion: 0.14
Nodes (26): From, discover(), discover_in(), discovery_reports_why_a_player_is_not_adapted(), Found, kugou_position(), live_kugou_discovery_reason(), Miss (+18 more)

### Community 69 - "当前媒体与曲目间隔"
Cohesion: 0.11
Nodes (25): complete_songs_are_not_recorded_and_gaps_are_named(), each_song_is_judged_once_after_it_settles_while_playing(), gaps(), looks_like_program(), origin(), Instant, Option, String (+17 more)

### Community 70 - "网易云封面与详情缓存"
Cohesion: 0.14
Nodes (28): cover_failure_can_retry_without_a_minute_of_blank_artwork(), cover_url(), CoverCache, detail_from(), detail_response_fills_metadata_only_for_the_same_song(), DetailCache, fetch_cover(), fetch_detail() (+20 more)

### Community 71 - "日历兼容与事件解析"
Cohesion: 0.11
Nodes (27): discovery_keeps_root_events_and_follows_calendar_home_for_every_caldav_provider(), external_ical_properties_and_components_are_case_insensitive(), import_report_explains_invalid_outside_cancelled_and_non_event_records(), inaccessible_home_does_not_block_other_homes_or_hide_the_warning(), malformed_recurrence_override_does_not_restore_the_unmodified_series(), malformed_single_event_keeps_other_valid_events(), mixed_caldav_response_preserves_readable_events_and_reports_resource_errors(), mock_fetch() (+19 more)

### Community 72 - "Claude 桌面缓存"
Cohesion: 0.16
Nodes (26): body(), Cache, desktop_cache_decodes_compressed_responses_and_rejects_damage_or_excessive_output(), desktop_cache_keeps_missing_expired_and_unzoned_resets_unknown(), desktop_cache_reads_files_held_open_with_delete_access(), desktop_cache_reads_official_reset_times_without_changing_files(), desktop_cache_rejects_other_organizations_and_old_or_future_responses(), desktop_cache_supports_limit_rows_and_never_infers_missing_windows() (+18 more)

### Community 73 - "通知绘制与健康提醒"
Cohesion: 0.12
Nodes (11): Alert, device_icon(), HealthReminder, notice_badge(), notice_button(), notice_icon(), Cmd, Into (+3 more)

### Community 74 - "酷狗曲目信息"
Cohesion: 0.14
Nodes (20): live_kugou_forced_wraparound_finds_the_same_duration(), log_scan(), own_process(), planted(), Process, read_slice(), read_slice_in(), Region (+12 more)

### Community 75 - "歌词来源与匹配"
Cohesion: 0.18
Nodes (29): artist_set(), bare_title(), clean_title(), credit_match(), fetch_from(), fetch_netease(), fetch_netease_song(), FetchError (+21 more)

### Community 76 - "CalDAV 请求与错误"
Cohesion: 0.24
Nodes (19): agent(), dav(), dav_response(), discover(), discover_at(), discover_calendars(), Failure, fetch() (+11 more)

### Community 77 - "功能截图与界面回归"
Cohesion: 0.11
Nodes (23): calendar_partial_sync_preview(), enable(), five_client_icons_fit_collapsed_and_expand_to_their_own_click_targets(), messenger_brand_icon_gallery(), office_notice_gallery(), office_reminders_keep_instances_separate_and_unknown_accounts_require_explicit_dismissal(), office_taskbar_mode_deduplicates_system_toasts_without_disabling_the_existing_windows_mode(), App (+15 more)

### Community 78 - "设置窗口位置"
Cohesion: 0.13
Nodes (16): Area, Fit, invalid_window_records_fall_back_without_rewriting_them(), Placement, Context, Drop, Option, PathBuf (+8 more)

### Community 79 - "豆包额度接口测试"
Cohesion: 0.14
Nodes (22): activity_only_and_empty_plan_names_are_supported(), agent(), amount_fallback_and_zero_timestamps_stay_honest(), config_write_failure_keeps_the_original_session_and_removes_the_unused_one(), cookie_requires_a_unique_session_and_rejects_header_injection(), dropping_a_connection_cancels_it_and_busy_validation_does_not_spawn_more_workers(), encrypted_storage_and_duplicate_reconnection_preserve_account_settings(), groups_keep_both_cycles_and_every_extra_pool() (+14 more)

### Community 80 - "额度页面交互测试"
Cohesion: 0.08
Nodes (26): clock_12h_formats_boundaries_and_persists(), deepseek_balance_gallery(), deepseek_compact_amount_fits_text_and_opens_account(), deepseek_resident_preview_does_not_overlap_icons(), kiro_cancelled_or_closed_connections_cannot_save_late_results(), kiro_connection_save_failure_keeps_the_original_account_and_cache(), kiro_credits_follow_the_ring_on_the_resident_bar(), kiro_logo_and_point_details_fit_the_card() (+18 more)

### Community 81 - "启动诊断日志"
Cohesion: 0.15
Nodes (22): diagnostics_dir(), directory(), failure_creates_a_report_and_exposes_its_location(), filter(), graphics(), install(), message(), module_at() (+14 more)

### Community 82 - "播放器独立音量"
Cohesion: 0.13
Nodes (20): ISimpleAudioVolume, Job, Latest, apply(), executable(), live_windows_sessions_change_together_without_changing_master_volume(), read(), Reader (+12 more)

### Community 83 - "酷狗目录缓存"
Cohesion: 0.15
Nodes (24): alias_header_requires_exact_filename_and_opening_credit(), cached_duet_identity_ignores_order_and_credit_field(), configured_unicode_directory_and_exact_track_selection(), decode(), directory_from_ini(), duet_cache_accepts_primary_artist_only_when_filename_matches_full_track(), empty_headers_use_exact_filename_without_accepting_other_versions(), fixture() (+16 more)

### Community 84 - "和风天气客户端"
Cohesion: 0.20
Nodes (16): process_error(), all_endpoints_share_budget_disabled_calls_never_send_and_key_is_not_in_url(), Client, directory(), Error, monthly_limit_persists_handles_concurrency_and_resets_next_month(), normalize_host(), reserve() (+8 more)

### Community 85 - "和风天气数据解析"
Cohesion: 0.22
Nodes (18): attributions(), Cache, cache_is_host_and_location_scoped_and_module_off_discards_inflight_results(), current(), current_refresh_failure_requires_repeated_errors_and_recovers_on_success(), daily(), display_only_rebuilds_on_new_data_or_city_midnight(), fresh_short_forecast_obeys_the_success_interval() (+10 more)

### Community 86 - "日历配置与同步"
Cohesion: 0.21
Nodes (22): Config, config_path(), decode_config(), delete(), encode_config(), forget_config(), live_calendar_discovery_read_only_probe(), live_calendar_read_only_probe() (+14 more)

### Community 87 - "媒体控制工作线程"
Cohesion: 0.14
Nodes (18): ControlRequest, control(), control_hotkey(), live_native_netease_duration_and_progress(), ControlWorker, music_volume_queue_coalesces_only_the_same_players_volume_changes(), queued_seeks_for_the_same_player_run_only_the_latest(), resource_control_queue_stays_bounded_when_the_player_is_stuck() (+10 more)

### Community 88 - "邮件事件与解析状态"
Cohesion: 0.17
Nodes (16): body_of(), code_of(), display_addr(), email_links_preserve_targets_labels_and_query_parameters(), event_of(), ics_text(), Link, links_of() (+8 more)

### Community 89 - "自动避让选择器"
Cohesion: 0.17
Nodes (13): Candidate, contains_app(), Picker, Cache, Context, Option, Receiver, Rect (+5 more)

### Community 90 - "字体发现与加载"
Cohesion: 0.16
Nodes (19): FontDefinitions, IconData, alpha_blue(), app_icon(), build_fonts(), Face, find(), font_bytes() (+11 more)

### Community 91 - "毛玻璃配色与对比度"
Cohesion: 0.17
Nodes (13): backdrop_estimate(), backdrop_estimate_leans_toward_bright_samples(), contrast(), dark_tint(), glass_tint(), glass_tint_keeps_white_text_readable_on_any_backdrop(), light_glass_keeps_dark_text_readable(), light_tint() (+5 more)

### Community 92 - "健康提醒交互测试"
Cohesion: 0.17
Nodes (13): badge(), click_badge(), due(), health_actions_clear_pending_and_keep_messenger_unread(), health_badges_confirm_independently_and_do_not_record_water(), health_fullscreen_hidden_keeps_pending_silent_and_returns_as_badges(), health_pending_survives_restart_and_day_change(), health_reminder_gallery() (+5 more)

### Community 93 - "应用计时与日历事件"
Cohesion: 0.12
Nodes (5): Phase, Pomo, Config, Event, Into

### Community 94 - "日历平台与诊断信息"
Cohesion: 0.11
Nodes (11): calendar_data_request(), human_error(), Provider, report_body(), report_requests_expanded_recurring_events(), Url, same_service(), selected_provider() (+3 more)

### Community 95 - "演示录制脚本"
Cohesion: 0.22
Nodes (15): Act, cursor_shapes(), ease(), gallery(), main(), Media, p(), Option (+7 more)

### Community 96 - "日历同步调度"
Cohesion: 0.19
Nodes (15): automatic_sync_can_stop_resume_and_leave_manual_sync_available(), Cmd, Event, month_grid(), month_grid_covers_leap_days_and_year_changes(), month_start(), multiple_calendars_sync_even_when_one_fails(), Duration (+7 more)

### Community 97 - "媒体封面搜索"
Cohesion: 0.18
Nodes (15): cover_waits_for_a_free_worker_without_spending_an_attempt(), CoverSearch, pick(), FnOnce, Instant, JoinHandle, Option, Receiver (+7 more)

### Community 98 - "播放器进度定位"
Cohesion: 0.20
Nodes (16): IUIAutomationRangeValuePattern, apple_bar(), apple_value(), automation(), DpiContext, matches_player(), qq_bar(), DPI_AWARENESS_CONTEXT (+8 more)

### Community 99 - "歌词缺失缓存"
Cohesion: 0.20
Nodes (19): cache_path(), cached_lyrics_preserve_timing_and_invalid_caches_are_ignored(), confirmed_missing_lyrics_are_remembered_for_a_week_but_failures_are_not(), fetch_for_source(), fetch_online(), fetch_online_detail(), from_plain(), live_kugou_uses_native_timestamps() (+11 more)

### Community 100 - "软件光栅绘制"
Cohesion: 0.17
Nodes (11): Raster, HashMap, Option, Self, TextureId, Vec, sample(), Tex (+3 more)

### Community 101 - "IMAP 连接与超时"
Cohesion: 0.18
Nodes (14): connect(), idle_without_push_returns_for_a_fallback_check(), Prefixed, Prefixed<S>, Prefixed<Stream>, probe(), read_line(), Duration (+6 more)

### Community 102 - "邮箱账户与服务"
Cohesion: 0.21
Nodes (13): Account, Cmd, decode_accounts(), exmail_live_capability_probe(), key(), load(), load_accounts(), qq_live_calendar_mail_probe() (+5 more)

### Community 103 - "待办提醒选择器"
Cohesion: 0.24
Nodes (9): NaiveDateTime, date_wheel_stops_at_today_and_ignores_date_overflow(), PickerAction, ReminderPicker, NaiveDate, Option, Self, String (+1 more)

### Community 104 - "桌面背景采样"
Cohesion: 0.17
Nodes (11): Arc, AtomicBool, Drop, Mutex, Option, Self, Vec, sample() (+3 more)

### Community 105 - "日历发现回归测试"
Cohesion: 0.19
Nodes (14): caldav_collection_listing_reads_slash_objects_and_recurring_events(), discovery_authentication_failure_does_not_probe_other_paths(), discovery_errors_at_resolved_service_preserve_status_and_stop(), discovery_principal_cannot_send_credentials_to_another_origin(), discovery_root_errors_follow_well_known_principal_and_home(), fetch_month(), month_query_fetches_past_dates_and_excludes_the_next_month(), query_missing_data_retries_multiget_and_rejects_nested_errors() (+6 more)

### Community 106 - "在线歌词查询测试"
Cohesion: 0.15
Nodes (15): fallback_checks_identity_even_when_wrong_recording_has_exact_duration(), fetch_qqmusic(), live_qq_featured_track(), live_qq_uncached_tracks(), matching_unifies_featured_credits_across_sources_without_losing_versions(), netease_search_retries_with_title_only_when_combined_query_misses(), netease_song_id_fetches_lyrics_without_a_search_request(), online_candidates_continue_after_one_lyric_request_fails() (+7 more)

### Community 107 - "歌词来源回退"
Cohesion: 0.22
Nodes (14): fetch_for_display(), fetch_for_display_with(), fetch_with_sources(), fetch_with_sources_detail(), fetch_with_sources_once(), live_chinese_tracks(), live_kugou_short_pause_stays_on_previous_line(), lookup_steps_are_traced_for_the_local_diagnostic_log() (+6 more)

### Community 108 - "邮箱服务端配置"
Cohesion: 0.15
Nodes (5): CustomImap, microsoft_cloud_accounts_report_unsupported_before_connecting(), Provider, Default, Self

### Community 109 - "酷狗曲目缓存"
Cohesion: 0.34
Nodes (9): Cache, current(), kugou_now(), kugou_windows(), Instant, Option, String, track() (+1 more)

### Community 110 - "歌词切换回归测试"
Cohesion: 0.46
Nodes (13): busy(), completed_artist_credits_requery_but_keep_showing_current_lyrics(), filled_in_duration_keeps_synced_lyrics_without_a_new_lookup(), finish(), lyrics_on(), native_lyrics_polling_starts_on_switch_without_waiting_for_the_old_request(), H, Sender (+5 more)

### Community 111 - "侧边停靠几何"
Cohesion: 0.26
Nodes (4): App, Pos2, Rect, Vec

### Community 112 - "日历 TLS 传输测试"
Cohesion: 0.22
Nodes (9): calendar_agent_uses_system_tls_and_rejects_untrusted_certificate_before_http(), certificates(), Agent, JoinHandle, String, Value, server(), trusted_client() (+1 more)

### Community 113 - "酷狗播放器控制"
Cohesion: 0.26
Nodes (10): endpoint(), player(), read_endpoint(), Option, Path, PathBuf, String, Value (+2 more)

### Community 114 - "图形渲染回退"
Cohesion: 0.21
Nodes (7): apply_pending_fallback(), fallback_marker_turns_on_compat_rendering_once(), marker(), request_fallback(), Option, PathBuf, String

### Community 115 - "豆包额度界面测试"
Cohesion: 0.26
Nodes (9): doubao_activity_is_visible_on_the_island_and_in_account_details(), doubao_connection_offers_official_login_without_an_extension(), doubao_quota_gallery(), fixture(), has_text(), Event, H, Vec (+1 more)

### Community 116 - "音乐诊断日志"
Cohesion: 0.27
Nodes (9): changed(), file_version(), player(), player_exe(), reads_the_version_of_a_system_file(), Option, String, state() (+1 more)

### Community 117 - "和风天气刷新调度"
Cohesion: 0.21
Nodes (7): FnOnce, NaiveDate, Receiver, Send, T, Runtime, spawn()

### Community 118 - "设置数值与健康时段"
Cohesion: 0.27
Nodes (9): DragValue, hhmm(), island_number_field(), island_switch(), Hash, Response, setting_button(), settings_toggle_chip() (+1 more)

### Community 119 - "番茄钟绘制"
Cohesion: 0.36
Nodes (10): draw(), fill_polygon(), liquid(), liquid_remains_bounded_when_full_nearly_empty_or_empty(), remaining_energy_area_decreases_with_time(), Color32, Vec, tea() (+2 more)

### Community 120 - "播放器进程识别"
Cohesion: 0.24
Nodes (7): exe_name(), image_is(), image_path(), pids_named(), Option, String, Vec

### Community 121 - "和风天气配置与用量"
Cohesion: 0.24
Nodes (7): Config, month(), Provider, String, Vec, Search, Usage

### Community 123 - "天气城市设置"
Cohesion: 0.33
Nodes (7): settings_hint(), settings_label(), city_option(), info_row(), FnOnce, Response, Ui

### Community 125 - "系统通知来源"
Cohesion: 0.39
Nodes (6): Option, Path, String, sender(), sender_in(), sender_lookup_matches_creation_time_and_never_modifies_the_database()

### Community 126 - "邮件操作交互测试"
Cohesion: 0.25
Nodes (8): local_reader_moves_and_restores_after_switching_the_main_list(), mail_local_mode_persists_without_writing_to_the_server(), managed_mail_actions_wait_for_the_server_and_batch_read_is_explicit(), managed_mail_fixture(), managed_mail_trash_icon_and_restore_keep_inbox_uid_separate(), managed_mail_unread_filter_open_and_mark_read(), new_mail_preserves_an_explicit_reveal_in_the_trash(), reader_actions_remain_bound_to_the_opened_mailbox()

### Community 127 - "本机凭据解密"
Cohesion: 0.38
Nodes (6): decrypt(), key(), Path, Result, String, Vec

### Community 128 - "侧边常驻栏绘制"
Cohesion: 0.33
Nodes (3): String, side_clock(), SideItem

### Community 130 - "子进程启动"
Cohesion: 0.40
Nodes (5): lightweight_child_stdio_does_not_keep_parent_pipes_open(), Child, Result, spawn(), Command

### Community 131 - "本地歌词指纹"
Cohesion: 0.33
Nodes (5): is_soda(), native_fingerprint(), read_native(), read_native_detail(), SystemTime

### Community 132 - "多语言字形支持"
Cohesion: 0.40
Nodes (6): font(), korean_titles_and_lyrics_have_glyphs_in_all_weights(), load_fonts(), Context, FontId, uyghur_and_tibetan_lyrics_have_glyphs_in_all_weights()

### Community 133 - "映射字体文件"
Cohesion: 0.33
Nodes (5): MappedFont, File, Mmap, Result, Self

### Community 135 - "歌词缺失缓存开关"
Cohesion: 0.40
Nodes (3): Drop, Self, SkipNegative

### Community 136 - "控件拖动排序测试"
Cohesion: 0.40
Nodes (5): point(), RECT, avoidance_picker_add_remove_and_save(), page_order_drag_applies_on_release_and_saves(), resident_components_can_be_dragged_saved_and_reset()

## Knowledge Gaps
- **6 isolated node(s):** `Config`, `Region`, `App`, `crate::app::App`, `Class` (+1 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Shell 如何启动蓝牙监听并把设备状态交给 App？**
  _Shell 与蓝牙设备社区之间存在类型引用，源码可核对服务创建和消息接收。_
- **App 如何将设备变化转成通知，并处理蓝牙音频与音量提示的优先级？**
  _App 连接蓝牙、逐帧绘制、通知和系统媒体等社区，介数中心性为 0.166。_
- **NowPlaying 如何连接系统媒体、播放器适配、封面和歌词？**
  _NowPlaying 的介数中心性为 0.135，是多个媒体社区的桥接节点。_