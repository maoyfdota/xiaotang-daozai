# macOS 性能页技术设计

## 边界

最小行为差距是 Mac 当前没有性能页，也没有可供该页绘制趋势的采样数据。数据采集放在独立的 `performance` 模块，页面绘制放在 `performance_ui`，页面是否可达仍由 `platform::page_available` 决定。

预期修改：

| 文件 | 原因 |
| --- | --- |
| `src/performance.rs`、`src/performance/macos.rs` | 指标值、采样差值、历史样本与 macOS 原生计数器采样 |
| `src/performance_ui.rs` | 四类指标卡和趋势线；不可用值保留为空 |
| `src/app.rs`、`src/lib.rs` | 注册页 ID、状态、采样节奏、标题／高度与页面渲染 |
| `src/model.rs` | 扩展持久化页面开关和顺序，迁移旧配置 |
| `src/platform.rs` | 只在 macOS 开放页面 ID 10 和设置页签 ID 16 |
| `src/extras.rs`、`assets/settings-icons/system/performance.svg` | 页面管理描述、功能设置入口、页签名称与图标 |
| `.trellis/spec/backend/macos-platform.md` | 记录性能采样及页面能力契约 |
| 相关 Rust 测试 | 覆盖已确认的页面／迁移、计数器差值和不可用边界 |

不改 Windows 的性能或渲染后端，不添加进程枚举、GPU、温度、常驻组件，不调整蓝牙 ID／设置和已完成的设备行为。

## 数据与采样契约

- `Reading` 的 CPU、内存、磁盘、网络字段均为 `Option`；未知与失败一直保持 `None`。
- CPU 使用 Mach 主机逐核累计时间，取两次快照中 busy／total 的差值。初始快照只建立基线。
- 物理内存由 Mach VM 计数器和 `hw.memsize` 推导；总量为零或任一接口失败时返回不可用。
- 磁盘累计读写字节从 IOKit `IOBlockStorageDriver` 的 `Statistics` 字典读取；网络收发从活动非回环接口累计字节读取。
- 磁盘和网络速率为 `(当前累计值 - 上次累计值) / 实际经过秒数`。首次样本、非正间隔、累计值回退或数据源缺失返回不可用并重建基线，不填零。
- 页面更新周期约一秒；趋势最多保存六十个时点。某个时点缺少某项指标时，曲线断开。

## 配置兼容

- `PAGE_NAMES`、可见性数组和顺序数组追加 ID 10；所有原 ID 固定。
- `page_order` 接受 8–11 个有效完整排列；较短旧排列复制原序，新 ID 保持默认排列位置（末尾）。
- `visible_pages` 接受 6–11 位；原位逐项保留，旧配置缺少的新增页按已有迁移规则默认开启。
- 设置页签 ID 16 映射到页面 10；Mac 能访问，其他平台不可访问。原标签映射不移动。

## 验证与回退

按用户确认的测试边界编写行为测试。先对页面索引／配置、采样差值和不可用状态做红绿测试，再完成界面接入并运行定向单测、Mac arm64 release 检查。发现原生采样接口缺失时，该指标显示不可用；不以静态估值或零值替代。回退时可单独关闭页面可用性并撤销性能模块接入；配置解析保留未知旧值，不删除蓝牙配置。
