# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Nexus Core is a Tauri v2 desktop network proxy management tool for IT professionals and network engineers. Frontend: React 18 + TypeScript + Vite + Tailwind CSS. Backend: Rust with ~50 modules organized by network subsystem. UI is Chinese-first with i18n support, WCAG AA contrast, dark/light themes.

## Build & Dev Commands

```bash
# Frontend
npm install              # install deps from package-lock.json
npm run dev              # Vite dev server on :5173
npm run build            # tsc type-check → vite build → dist/
npm run lint             # ESLint on src --ext .ts,.tsx
npm run format           # Prettier --write "src/**/*.{ts,tsx,css}"

# Backend
cd src-tauri && cargo check                           # type-check Rust
cd src-tauri && cargo test --lib                      # run Rust unit tests
cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings  # Rust lints
cd src-tauri && cargo fmt --all -- --check            # Rust format check
cd src-tauri && cargo audit                           # RustSec advisory check
cd src-tauri && cargo deny check                      # License + dependency audit

# Full Tauri app (frontend + backend together)
npx tauri dev                                         # starts Vite + launches desktop window
npx tauri build                                       # production bundle (all platforms)
```

**CI runs on push to main/develop and PRs:** frontend lint+build, Rust fmt/clippy/check on 3 platforms, `cargo test --lib`, release build check, security audit.

## Frontend Architecture

```
src/
├── app/          App.tsx, Providers (theme), Router (createBrowserRouter, 9 routes)
├── components/
│   ├── common/   17 reusable components (Button, Card, Modal, Table, Toast, Drawer,
│   │               Dropdown, Empty, Input, Loading, Pagination, Select, Sparkline, Toggle, Badge, Chart, ConfirmDialog)
│   └── layout/   MainLayout + Sidebar (collapsible, nav items from constants/navigation.ts)
├── pages/        9 route pages (each a directory with index.tsx):
│                 Dashboard, Profiles, Nodes, Rules, Connections, Logs, Statistics, Settings, About
├── stores/       9 Zustand stores — one per domain (profileStore, nodeStore, ruleStore,
│                 connectionStore, logStore, statisticsStore, dashboardStore, appStore, settingsStore)
│                 Each holds: data[], search/sort/pagination state, loading flag, CRUD actions
├── services/     8 services that return mock data from @/mock/seed.ts
│                 Designed to be swapped to @tauri-apps/api invoke() calls
├── mock/         seed.ts — mock data generators (SEED_PROFILES, SEED_NODES, SEED_RULES, etc.)
├── types/        domain.ts, store.ts, event.ts, component.ts, index.ts
├── hooks/        useTheme, useToast, useMockStream (simulates live data), usePolling
├── i18n/         zh-CN.ts, en.ts, types.ts, index.ts — translation dictionaries
├── constants/    navigation.ts (sidebar items), settings.ts (settings schema), theme.ts, index.ts
├── utils/        cn() (clsx+twMerge), formatters.ts, validators.ts, index.ts
└── styles/       index.css — CSS custom properties for theming, Tailwind base layers
```

**Key patterns:**
- `@/` path alias maps to `src/` (configured in both vite.config.ts and tsconfig.json)
- Service → Store → Component data flow; stores call services, components call stores
- `useMockStream` in App.tsx polls stores on a timer to simulate live network data
- All state is Zustand — no React Context except the theme hook in Providers
- Pages use directory-based routing (e.g., `pages/Dashboard/index.tsx`)

## Backend Architecture (Rust)

```
src-tauri/src/
├── main.rs           Windows no-console shim; delegates to nexus_core_lib::app::run()
├── lib.rs            Module declarations + hand-written prost types for geosite protobuf
├── app.rs            Boot sequence (documented inline), Tauri builder, IPC handler registration
├── core/             AppState, CoreManager, ResourceManager, Runtime, TaskManager
├── runtime/          RuntimeContext — central DI hub; ShutdownToken for graceful shutdown
├── engine/           Pluggable proxy engines: mihomo/, singbox/, xray/, native/, plugin/
├── protocol/         Protocol layer (Phase 4): inbound/outbound adapters, HTTP/SOCKS5
├── transport/        Transport layer (Phase 4): listener + stream abstractions
├── dispatcher/       Route dispatcher (Phase 4)
├── pipeline/         Packet pipeline (Phase 6): processors (log, statistics, echo)
├── proxy/            HTTP + SOCKS5 proxy (Phase 7)
├── tun/              TUN interface + route management (Phase 8)
├── dns/              DNS manager, cache, DoH/DoT resolvers, system resolver (Phase 9)
├── rule_engine/      Rule matching engine (Phase 9): domain/IP/port/keyword/suffix matchers
├── subscription/     Subscription management (Phase 11): clash/singbox/base64 parsers
├── ruleset/          RuleSet download/reload (Phase 11)
├── core_installer/   Engine binary download, version management, update, rollback (Phase 13)
├── geo/              GeoIP/GeoSite with MaxMind DB + protobuf (Phase 14)
├── telemetry/        Crash count, startup duration, memory sampling (Phase 15)
├── security/         Security audit, path/download validation (Phase 15)
├── performance/      Benchmark, stress test, memory report (Phase 15)
├── release/          App updater (Phase 15)
├── config/           TOML config manager with file watching
├── storage/          Database abstraction (SQLite via rusqlite + r2d2 pool)
├── repository/       Domain repositories (Profile, Rule, Settings, Statistics) — trait + impl
├── service/          Backend services (dashboard, node, profile, rule, connection, log, etc.)
├── ipc/              Tauri command handlers — one fn per IPC call, registered in app.rs
├── event/            EventBus, BackendEmitter — push events to frontend
├── tray/             System tray icon + menu
├── backup/           Config backup + restore
├── diagnostics/      Crash reports, health reports, system reports
├── migration/        Schema/data migration
├── recovery/         Error recovery
├── connection/       Connection state management
├── monitoring/       Connection monitor
├── network/          Network state + engine abstractions
├── node/             Node manager
├── profile/          Profile management
├── rule/             Rule manager
├── session/          Session state + session manager
├── tunnel/           Tunnel trait + tunnel manager
├── statistics/       Traffic monitor
├── logging/          Logging subsystem
├── platform/         Platform-specific ops (Windows impl)
├── models/           Shared domain models (Profile, Node, Connection, Rule, LogEntry, etc.)
└── utils/            AppResult/AppError types, error handling helpers
```

**Key patterns:**
- Every subsystem follows: **Context (DI) → Manager (logic) → State (data)**
- `RuntimeContext` is the central dependency injection hub — all managers are stored here and accessed via getter/setter methods
- `ResourceManager` owns lifecycle of ConfigManager, Database, EventBus, TaskManager, PlatformManager, and all Repositories — initialized in dependency order
- Boot sequence in `app.rs::run()` builds all Phase 3–15 subsystems, wires them into RuntimeContext, then launches Tauri
- IPC commands are plain async Rust fns registered via `tauri::generate_handler![]` (~80 handlers)
- SQLite via rusqlite with r2d2 pool; repositories use traits (`ProfileRepository`, `RuleRepository`, etc.) implemented by `Sqlite*Repository`
- Config in TOML format, stored in app data dir, watched for changes via notify crate
- Engines (mihomo/singbox/xray) are external binaries managed as child processes
- Unit tests in `#[cfg(test)]` blocks across ~100 files; `ResourceManager::new_for_test()` provides in-memory SQLite for testing

## Frontend → Backend Bridge

Frontend services currently return mock data from `@/mock/seed.ts`. When wiring real IPC:
1. Each service method calls `invoke('<command_name>', { args })` from `@tauri-apps/api`
2. IPC command names follow `snake_case` matching the handler fns in `src-tauri/src/ipc/`
3. All registered IPC commands are listed in `app.rs` inside `generate_handler![]`
4. Tauri state is managed via `app.manage()` — CoreManager and AppState are managed state

## Design Principles (from PRODUCT.md)

- Functional-first: UI serves operational efficiency; decoration yields to information
- Quiet and restrained: color and motion only where meaningful, never purely decorative
- Desktop-native feel: should feel like a macOS/Windows native app, not a web wrapper
- At-a-glance: critical info is never hidden; shortest operation path
- WCAG AA contrast, dark/light theme, Chinese interface
