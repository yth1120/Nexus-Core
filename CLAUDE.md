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
npm run lint             # ESLint on src/**/*.{ts,tsx}
npm run format           # Prettier on src/**/*.{ts,tsx,css}

# Backend
cd src-tauri && cargo check                           # type-check Rust
cd src-tauri && cargo test                            # run Rust tests
cd src-tauri && cargo clippy --all-targets --all-features  # Rust lints
```

Tauri dev (full app): `npm run dev` starts Vite, then `cargo tauri dev` from `src-tauri/` launches the desktop window. The `beforeDevCommand` in `tauri.conf.json` auto-starts Vite.

## Frontend Architecture

```
src/
├── app/          App, Providers (theme), Router (createBrowserRouter, 9 routes)
├── components/
│   ├── common/   14 reusable components (Button, Card, Modal, Table, Toast, etc.)
│   └── layout/   MainLayout + Sidebar (collapsible, nav items from constants/navigation.ts)
├── pages/        9 route pages: Dashboard, Profiles, Nodes, Rules, Connections,
│                 Logs, Statistics, Settings, About
├── stores/       Zustand stores — one per domain (profileStore, nodeStore, etc.)
│                 Each holds: data[], search/sort/pagination state, loading flag, CRUD actions
├── services/     Data layer — currently returns mock data from mock/seed.ts
│                 Designed to be swapped to @tauri-apps/api invoke() calls
├── types/        domain.ts (Profile, Node, Connection, Rule, etc.), store.ts, event.ts
├── hooks/        useTheme, useToast, useMockStream (simulates live data), usePolling
├── i18n/         zh-CN.ts, en.ts, types.ts — translation dictionaries
├── constants/    navigation.ts (sidebar items), settings.ts (settings schema), theme.ts
├── utils/        cn() (clsx+twMerge), formatters.ts, validators.ts
└── styles/       index.css — CSS custom properties for theming, Tailwind base layers
```

**Key patterns:**
- `@/` path alias maps to `src/` (configured in both vite.config.ts and tsconfig.json)
- Service → Store → Component data flow; stores call services, components call stores
- `useMockStream` in App.tsx polls stores on a timer to simulate live network data
- All state is Zustand — no React Context except the theme hook in Providers

## Backend Architecture (Rust)

```
src-tauri/src/
├── app.rs           Boot sequence (documented inline), setup(), IPC handler registration
├── lib.rs           Module declarations + hand-written prost types for geosite protobuf
├── core/            AppState, CoreManager, ResourceManager, Runtime, TaskManager
├── runtime/         RuntimeContext — central DI hub; every manager gets wired through it
├── engine/          Pluggable proxy engines: mihomo/, singbox/, xray/, native/, external/, plugin/
├── protocol/        Protocol layer (Phase 4)
├── transport/       Transport layer (Phase 4)
├── dispatcher/      Route dispatcher (Phase 4)
├── pipeline/        Packet pipeline (Phase 6)
├── proxy/           HTTP + SOCKS5 proxy (Phase 7)
├── tun/             TUN interface + route management (Phase 8)
├── dns/             DNS manager, cache, DoH/DoT resolvers, system resolver (Phase 9)
├── rule_engine/     Rule matching engine (Phase 9)
├── subscription/    Subscription management (Phase 11)
├── ruleset/         RuleSet download/reload (Phase 11)
├── core_installer/  Engine binary download, version management, update, rollback (Phase 13)
├── geo/             GeoIP/GeoSite with MaxMind DB + protobuf (Phase 14)
├── telemetry/       Crash count, startup duration, memory sampling (Phase 15)
├── security/        Security audit, path/download validation (Phase 15)
├── performance/     Benchmark, stress test, memory report (Phase 15)
├── release/         App updater (Phase 15)
├── config/          TOML config manager with file watching
├── storage/         SQLite via rusqlite + r2d2 connection pool
├── ipc/             Tauri command handlers — one fn per IPC call, registered in app.rs
├── event/           EventBus, BackendEmitter — push events to frontend
├── tray/            System tray icon + menu
├── backup/          Config backup + restore
├── diagnostics/     Crash reports, health reports, system reports
├── migration/       Schema/data migration
├── recovery/        Error recovery
└── models/          Shared domain models
```

**Key patterns:**
- Every subsystem follows: **Context (DI) → Manager (logic) → State (data)**
- `RuntimeContext` is the central dependency injection hub — all managers are stored here and accessed via getter/setter methods
- Boot sequence in `app.rs::run()` is numbered and documented inline
- IPC commands are plain async Rust fns registered via `tauri::generate_handler![]`
- SQLite via rusqlite with r2d2 pool; repositories in `storage/`
- Config in TOML format, stored in app data dir, watched for changes via notify crate
- Engines (mihomo/singbox/xray) are external binaries managed as child processes

## Frontend → Backend Bridge

Frontend services currently return mock data. When wiring real IPC:
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
