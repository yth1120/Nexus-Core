# Nexus Core
<img width="1912" height="948" alt="image" src="https://github.com/user-attachments/assets/11021fae-7e7c-437b-ac9c-7054b5b05359" />

桌面网络代理管理工具 — 面向 IT 专业人员与网络工程师的高性能桌面客户端。

**技术栈：** Rust + Tauri v2 · React 18 + TypeScript · Tailwind CSS · SQLite

**平台支持：** Windows · macOS · Linux  
**界面语言：** 中文（支持 i18n 扩展）  
**主题：** 浅色 / 深色 · WCAG AA 对比度

---

## 功能概览

| 模块 | 说明 |
|---|---|
| 📊 仪表盘 | 实时流量监控、CPU / 内存占用、运行时长、当前节点状态 |
| 📋 配置文件管理 | 多配置文件切换，支持 VLESS / VMess / Trojan / Shadowsocks / Clash Meta / WireGuard |
| 🌐 节点管理 | 节点延迟测试、收藏、分组、国家 / 地区筛选 |
| 🔗 规则引擎 | DNS 分流 / GeoIP / GeoSite 匹配，支持自定义规则优先级 |
| 📡 连接监控 | 活跃连接列表、进程识别、上下行速率、实时流量统计 |
| 📈 流量统计 | 今日 / 本月流量、日平均曲线、峰值速率记录 |
| 📜 日志 | 分级日志（TRACE → ERROR），实时滚动，支持关键字搜索 |
| ⚙️ 设置 | 开机启动、静默模式、混合端口、TUN 模式、DNS 配置、日志级别 |
| 🔄 订阅更新 | 远程订阅自动更新 / 规则集下载 |
| 🧪 引擎切换 | 支持 mihomo / sing-box / Xray 引擎热切换 |
| 🛡️ 安全审计 | 路径遍历检测、下载完整性验证、SHA-256 校验 |

---

## 环境要求

| 依赖 | 版本 |
|---|---|
| Node.js | ≥ 18 |
| Rust | stable (1.80+) |
| npm | ≥ 9 |

**Windows：** 需安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)（Windows 10+ 已内置）  
**macOS：** Xcode Command Line Tools（`xcode-select --install`）  
**Linux：** 需安装系统依赖（见下方）

```bash
# Ubuntu / Debian
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev
```

---

## 快速开始

```bash
# 1. 克隆仓库
git clone https://github.com/nexuscore/nexus-core.git
cd nexus-core

# 2. 安装前端依赖
npm install

# 3. 启动开发模式
npm run dev          # 启动 Vite 前端（:5173）
cd src-tauri && cargo run  # 启动 Tauri 桌面窗口

# 或一步启动（需安装 Tauri CLI）
npx tauri dev
```

首次编译 Rust 后端约需 5–10 分钟（下载并编译依赖）。后续增量编译约 10–30 秒。

---

## 生产构建

```bash
# 前端
npm run build                      # TypeScript 检查 + Vite 打包 → dist/

# 后端
cd src-tauri && cargo build --release   # 优化编译（LTO + strip）

# 完整安装包（需 Tauri CLI）
npm install --save-dev @tauri-apps/cli@^2
npx tauri build
```

产物位置：

| 平台 | 路径 |
|---|---|
| Windows | `src-tauri/target/release/bundle/msi/*.msi` |
| macOS | `src-tauri/target/release/bundle/dmg/*.dmg` |
| Linux | `src-tauri/target/release/bundle/appimage/*.AppImage` |

---

## 项目结构

```
nexus-core/
├── src/                          # React 前端
│   ├── app/                      # 应用入口、路由、Provider
│   ├── components/
│   │   ├── common/               # 通用 UI 组件（Button, Card, Modal, Table…）
│   │   └── layout/               # 主布局 + 侧边栏
│   ├── pages/                    # 9 个路由页面
│   ├── stores/                   # Zustand 状态管理（每领域一个 Store）
│   ├── services/                 # 数据层（当前使用 mock，预留 Tauri IPC 接口）
│   ├── types/                    # TypeScript 类型定义
│   ├── hooks/                    # 自定义 Hooks
│   ├── i18n/                     # 国际化（zh-CN / en）
│   ├── constants/                # 导航、设置、主题常量
│   ├── utils/                    # 工具函数
│   └── styles/                   # 全局样式 + CSS 变量主题系统
│
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── app.rs                # 启动入口 + IPC 注册（含完整启动流程注释）
│   │   ├── lib.rs                # 模块声明
│   │   ├── core/                 # AppState / CoreManager / ResourceManager
│   │   ├── runtime/              # RuntimeContext（依赖注入中心）
│   │   ├── engine/               # 可插拔代理引擎（mihomo / sing-box / Xray / native）
│   │   ├── proxy/                # HTTP + SOCKS5 代理
│   │   ├── protocol/             # 协议层
│   │   ├── transport/            # 传输层
│   │   ├── dispatcher/           # 路由调度
│   │   ├── pipeline/             # 数据包管道
│   │   ├── tun/                  # TUN 虚拟网卡
│   │   ├── dns/                  # DNS 管理 + 缓存
│   │   ├── rule_engine/          # 规则引擎（DomainSuffix / GeoIP / GeoSite）
│   │   ├── subscription/         # 订阅管理
│   │   ├── ruleset/              # 规则集管理
│   │   ├── core_installer/       # 引擎下载 / 安装 / 更新 / 回滚
│   │   ├── geo/                  # GeoIP / GeoSite（MaxMind DB + Protobuf）
│   │   ├── config/               # TOML 配置管理 + 文件监听
│   │   ├── storage/              # SQLite（rusqlite + r2d2 连接池）
│   │   ├── ipc/                  # Tauri IPC 命令处理器
│   │   ├── event/                # 事件总线（Backend → Frontend 推送）
│   │   ├── telemetry/            # 遥测（启动耗时、崩溃计数、内存采样）
│   │   ├── security/             # 安全审计、路径验证、下载校验
│   │   ├── performance/          # 基准测试、压力测试、内存分析
│   │   ├── release/              # 应用更新器
│   │   ├── tray/                 # 系统托盘
│   │   ├── backup/               # 配置备份 / 恢复
│   │   └── diagnostics/          # 崩溃报告 / 健康检查
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── dist/                         # 前端构建产物
├── .github/workflows/            # CI / CD 工作流
├── CLAUDE.md                     # Claude Code 参考文档
├── AGENTS.md                     # 代码库规范
├── PRODUCT.md                    # 产品定义
└── deny.toml                     # cargo-deny 配置
```

---

## 架构设计

### 前端：Service → Store → Component

```
Service (mock/API)  →  Zustand Store  →  React Component
     ↑                    ↑                    ↑
 数据获取层          状态管理（含分页/排序/筛选）   UI 渲染
```

- `@/` 路径别名 → `src/`
- 状态管理使用 Zustand（无 React Context 依赖，除主题 Hook）
- 页面通过 `react-router-dom` v6 路由，`MainLayout` 提供统一的侧边栏布局
- 9 个路由：Dashboard / Profiles / Nodes / Rules / Connections / Logs / Statistics / Settings / About

### 后端：Context（DI） → Manager → State

```
RuntimeContext (DI 中心)
    ↓
XxxContext (子系统配置)  →  XxxManager (业务逻辑)  →  XxxState (状态)
```

- **RuntimeContext**：中央依赖注入容器，所有 Manager 通过它获取共享依赖
- **CoreManager**：顶级编排器，IPC 命令通过它路由到各子系统
- **EventBus**：`broadcast` 通道，后端事件同步推送到前端
- **TaskManager**：结构化后台任务管理（spawn / stop / restart / shutdown_all）
- **ShutdownToken**：`CancellationToken` 树，优雅关闭所有子系统

### 启动流程（`app.rs::run()`）

1. 初始化日志（`tracing` + 滚动文件追加器）
2. 解析应用数据目录
3. 创建 AppState
4. 构建 ResourceManager（Config + DB + EventBus + Tasks + Platform + Repos）
5. 按 Phase 依次构建所有子系统（Phase 3–15）
6. 启动后台运行时任务 + 系统托盘

---

## 开发指南

```bash
# 前端
npm run lint        # ESLint 检查
npm run format      # Prettier 格式化

# 后端
cd src-tauri
cargo fmt --check                       # 格式检查
cargo clippy --all-targets -- -D warnings  # Clippy 严格模式
cargo check --all-targets               # 类型检查（比 build 快）
cargo test --lib                        # 运行测试

# 安全审计
cargo install cargo-audit cargo-deny
cargo audit
cargo deny check
```

### 代码规范

- **TypeScript：** 函数组件 + PascalCase，camelCase Hooks，2 空格缩进，单引号，行末分号
- **Rust：** `rustfmt` 默认，snake_case 模块 / 函数，PascalCase 类型
- **错误处理：** 统一使用 `AppResult<T>`（`thiserror` 派生），禁止生产代码使用 `unwrap()`

---

## CI / CD

| 工作流 | 触发条件 | 内容 |
|---|---|---|
| `ci.yml` | push main/develop, PR | 前端 lint + build → Rust fmt/clippy/check → test → build-check → 安全审计 |
| `release.yml` | `v*` tag, 手动触发 | 4 平台构建（Win x64 / macOS x64+arm64 / Linux x64）→ SHA256 校验 → GitHub Release |

详情见 `.github/workflows/`。

---

## 安全

- **路径验证：** 防止目录遍历 + ZIP slip 攻击
- **下载校验：** 强制 SHA-256 完整性验证（不可跳过）
- **进程参数：** 命令注入检测（shell 元字符拦截）
- **依赖审计：** `cargo audit` + `cargo deny`（CI 阻塞级）
- **CSP：** Tauri `tauri.conf.json` 中配置内容安全策略
- **遥测：** 所有数据仅存储本地，不上传

---

## 设计原则

1. **功能优先** — 界面服务于操作效率，装饰让位于信息
2. **安静克制** — 色彩和动效只用在意有所指之处
3. **桌面原生感** — 如 macOS / Windows 原生应用，非网页套壳
4. **一目了然** — 关键信息不隐藏，操作路径最短

---

## 路线图

当前版本：**v2.4.1**（Phase 0–15 全部完成）

| 阶段 | 内容 | 状态 |
|---|---|---|
| Phase 3 | 网络核心（CoreManager + RuntimeContext） | ✅ |
| Phase 4 | 协议层 + 传输层 + 调度器 | ✅ |
| Phase 5 | 可插拔引擎层（mihomo / sing-box / Xray） | ✅ |
| Phase 6 | 数据包管道 | ✅ |
| Phase 7 | HTTP + SOCKS5 代理 | ✅ |
| Phase 8 | TUN 虚拟网卡 + 路由管理 | ✅ |
| Phase 9 | DNS 管理 + 规则引擎 | ✅ |
| Phase 11 | 订阅管理 + 规则集 | ✅ |
| Phase 13 | 核心安装器（下载 / 更新 / 回滚） | ✅ |
| Phase 14 | GeoIP / GeoSite | ✅ |
| Phase 15 | 遥测 + 安全 + 性能 + 应用更新 | ✅ |
| v1.0 beta | 发布就绪检查 | 🔄 |

---

## 许可

[MIT License](LICENSE)
