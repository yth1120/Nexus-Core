use std::sync::Arc;

use tauri::Manager;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::core::resource_manager::InitConfig;
use crate::core::{AppState, ResourceManager, Runtime};
use crate::tray::TrayManager;

/// Main entry point for the Nexus Core application.
///
/// Boot sequence:
/// 1. Initialize logging (tracing with rolling appenders)
/// 2. Resolve app data directory
/// 3. Create AppState (empty, no seed data — DB provides persistence)
/// 4. Build Tauri app with IPC, tray, and background tasks
/// 5. ResourceManager orchestrates Config+DB+EventBus+Tasks+Platform+Repos
pub fn run() {
    // ===== 1. Initialize logging =====
    let project_dirs = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
        .expect("Failed to determine app data directory");

    let app_dir = project_dirs.data_dir().to_path_buf();
    let log_dir = app_dir.join("logs");

    // Ensure log directory exists
    std::fs::create_dir_all(&log_dir).ok();

    let _log_guards = init_logging(&log_dir);

    tracing::info!("Nexus Core v2.4.1 starting...");
    tracing::info!("App data directory: {:?}", app_dir);

    // ===== 2. Create AppState (resource manager slot will be filled later) =====
    let app_state = Arc::new(AppState::new());
    tracing::info!("AppState initialized");

    // ===== 3. Build and run Tauri =====
    let app_state_for_setup = app_state.clone();
    let app_dir_for_setup = app_dir.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Dashboard
            crate::ipc::get_dashboard_status,
            crate::ipc::get_traffic_history,
            // Profiles
            crate::ipc::get_profiles,
            crate::ipc::get_profile,
            crate::ipc::create_profile,
            crate::ipc::update_profile,
            crate::ipc::delete_profile,
            crate::ipc::toggle_profile,
            // Nodes
            crate::ipc::get_nodes,
            crate::ipc::toggle_node_favorite,
            crate::ipc::test_node_delay,
            crate::ipc::test_all_node_delay,
            crate::ipc::connect_node,
            crate::ipc::disconnect_node,
            // Connections
            crate::ipc::get_connections,
            crate::ipc::close_connection,
            crate::ipc::close_all_connections,
            // Logs
            crate::ipc::get_logs,
            crate::ipc::get_recent_logs,
            // Rules
            crate::ipc::get_rules,
            crate::ipc::create_rule,
            crate::ipc::update_rule,
            crate::ipc::delete_rule,
            crate::ipc::toggle_rule,
            // Statistics
            crate::ipc::get_statistics,
            // Settings
            crate::ipc::get_settings_defaults,
            crate::ipc::save_settings,
            crate::ipc::validate_setting,
            // Core (Phase 3 network core)
            crate::ipc::core_start,
            crate::ipc::core_stop,
            crate::ipc::core_restart,
            crate::ipc::get_core_status,
            crate::ipc::connect_profile,
            crate::ipc::disconnect_profile,
            crate::ipc::get_current_session,
            // Protocol (Phase 4)
            crate::ipc::get_protocol_status,
            crate::ipc::get_transport_status,
            // Engine (Phase 5)
            crate::ipc::get_engine_list,
            crate::ipc::get_current_engine,
            crate::ipc::switch_engine,
            crate::ipc::start_engine,
            crate::ipc::stop_engine,
            crate::ipc::restart_engine,
            crate::ipc::reload_engine,
            crate::ipc::get_engine_capabilities,
            crate::ipc::get_engine_status,
            // Pipeline (Phase 6)
            crate::ipc::get_pipeline_status,
            crate::ipc::get_packet_statistics,
            // Subscription (Phase 11)
            crate::ipc::add_subscription,
            crate::ipc::remove_subscription,
            crate::ipc::update_subscription,
            crate::ipc::update_all_subscriptions,
            crate::ipc::get_subscriptions,
            // RuleSet (Phase 11)
            crate::ipc::download_ruleset,
            crate::ipc::reload_rulesets,
            crate::ipc::get_rulesets,
            // DNS (Phase 9)
            crate::ipc::resolve_domain,
            crate::ipc::flush_dns_cache,
            crate::ipc::get_dns_status,
            // Rule engine (Phase 9)
            crate::ipc::reload_rules,
            crate::ipc::match_rule,
            crate::ipc::get_rule_statistics,
            // Proxy (Phase 7 / 7.5)
            crate::ipc::start_http_proxy,
            crate::ipc::stop_http_proxy,
            crate::ipc::start_socks5_proxy,
            crate::ipc::stop_socks5_proxy,
            crate::ipc::get_proxy_status,
            // TUN (Phase 8)
            crate::ipc::start_tun,
            crate::ipc::stop_tun,
            crate::ipc::restart_tun,
            crate::ipc::get_tun_status,
            crate::ipc::set_traffic_mode,
            crate::ipc::get_traffic_mode,
            // Core Installer (Phase 13)
            crate::ipc::install_core,
            crate::ipc::uninstall_core,
            crate::ipc::get_core_versions,
            crate::ipc::get_current_core_version,
            crate::ipc::check_core_update,
            crate::ipc::update_core,
            crate::ipc::switch_core_version,
            crate::ipc::rollback_core,
            crate::ipc::download_core,
            crate::ipc::get_download_progress,
            // GeoIP / GeoSite (Phase 14)
            crate::ipc::get_geo_status,
            crate::ipc::reload_geo_database,
            crate::ipc::update_geo_database,
            crate::ipc::match_geoip,
            crate::ipc::match_geosite,
            // Phase 15: Telemetry, Security, Performance, Release
            crate::ipc::get_telemetry_report,
            crate::ipc::get_startup_duration,
            crate::ipc::get_crash_count,
            crate::ipc::run_security_audit,
            crate::ipc::validate_path,
            crate::ipc::validate_download,
            crate::ipc::run_benchmark,
            crate::ipc::run_stress_test,
            crate::ipc::generate_memory_report,
            crate::ipc::check_app_update,
            crate::ipc::download_app_update,
            crate::ipc::apply_app_update,
            crate::ipc::rollback_update,
        ])
        .setup(move |app| {
            let handle = app.handle().clone();

            // Store AppHandle in state for event emission
            app_state_for_setup.set_app_handle(handle.clone());

            // ===== Initialize ResourceManager =====
            let rt = tokio::runtime::Handle::current();
            let init_config = InitConfig {
                app_dir: app_dir_for_setup.clone(),
                app_handle: handle.clone(),
            };

            let rm = rt
                .block_on(async { ResourceManager::initialize(init_config).await })
                .expect("Failed to initialize ResourceManager");

            let rm = Arc::new(rm);

            // Store ResourceManager in AppState for services to access
            app_state_for_setup.set_resource_manager(rm.clone());

            // ===== Build network core (Phase 3): RuntimeContext + CoreManager =====
            // The context injects shared dependencies into every network-core
            // manager. CoreManager is Tauri-managed state so the `core_*` IPC
            // commands can reach it. It is NOT auto-started — Phase 3 is
            // architecture/mock only; lifecycle is driven via IPC.
            let runtime_ctx = Arc::new(crate::runtime::RuntimeContext::new(
                app_state_for_setup.clone(),
                rm.clone(),
                Some(handle.clone()),
            ));

            // ===== Build Phase 4: protocol + transport + dispatcher =====
            let protocol_ctx = Arc::new(crate::protocol::ProtocolContext::new(runtime_ctx.clone()));
            let protocol_manager = Arc::new(crate::protocol::ProtocolManager::new(protocol_ctx));
            runtime_ctx.set_protocol_manager(protocol_manager);

            let transport_ctx =
                Arc::new(crate::transport::TransportContext::new(runtime_ctx.clone()));
            let transport_manager =
                Arc::new(crate::transport::TransportManager::new(transport_ctx));
            runtime_ctx.set_transport_manager(transport_manager);

            let dispatcher = Arc::new(crate::dispatcher::Dispatcher::new(runtime_ctx.clone()));
            runtime_ctx.set_dispatcher(dispatcher);

            // ===== Build Phase 5: pluggable engine layer =====
            let engine_ctx = Arc::new(crate::engine::EngineContext::new(runtime_ctx.clone()));
            let engine_manager = Arc::new(crate::engine::EngineManager::new(engine_ctx));
            runtime_ctx.set_engine_manager(engine_manager.clone());

            // ===== Build Phase 6: packet pipeline =====
            let pipeline_manager =
                Arc::new(crate::pipeline::PipelineManager::new(runtime_ctx.clone()));
            runtime_ctx.set_pipeline_manager(pipeline_manager.clone());

            // ===== Build Phase 7: proxy layer =====
            let config = app_state_for_setup.get_config();
            let proxy_manager = Arc::new(crate::proxy::ProxyManager::new(
                runtime_ctx.clone(),
                config.http_port,
                config.socks_port,
            ));
            runtime_ctx.set_proxy_manager(proxy_manager);

            // ===== Build Phase 8: TUN + route managers =====
            let tun_ctx = Arc::new(crate::tun::TunContext::new(runtime_ctx.clone()));
            let route_manager = tun_ctx.route_manager.clone();
            let tun_manager = Arc::new(crate::tun::TunManager::new(tun_ctx));
            runtime_ctx.set_tun_manager(tun_manager);
            runtime_ctx.set_route_manager(route_manager);

            // ===== Build Phase 9: DNS + Rule Engine =====
            let dns_ctx = Arc::new(crate::dns::DnsContext::new(runtime_ctx.clone()));
            let dns_manager = Arc::new(crate::dns::DnsManager::new(dns_ctx));
            runtime_ctx.set_dns_manager(dns_manager);

            let rule_cache = Arc::new(crate::rule_engine::RuleCache::new());
            let rule_ctx = Arc::new(crate::rule_engine::RuleContext::new(
                runtime_ctx.clone(),
                rule_cache,
            ));
            let rule_engine = Arc::new(crate::rule_engine::RuleEngineManager::new(rule_ctx));
            runtime_ctx.set_rule_engine(rule_engine);

            // ===== Build Phase 11: Subscription + RuleSet =====
            let sub_ctx = Arc::new(crate::subscription::SubscriptionContext::new(
                runtime_ctx.clone(),
            ));
            let subscription_manager =
                Arc::new(crate::subscription::SubscriptionManager::new(sub_ctx));
            runtime_ctx.set_subscription_manager(subscription_manager);

            let rs_ctx = Arc::new(crate::ruleset::RuleSetContext::new(runtime_ctx.clone()));
            let ruleset_manager = Arc::new(crate::ruleset::RuleSetManager::new(rs_ctx));
            runtime_ctx.set_ruleset_manager(ruleset_manager);

            // ===== Build Phase 13: Core Installer layer =====
            let engines_dir = app_dir_for_setup.join("engines");
            std::fs::create_dir_all(&engines_dir).ok();

            let core_registry = Arc::new(
                crate::core_installer::CoreRegistry::new(&engines_dir).unwrap_or_else(|e| {
                    tracing::error!("Failed to create CoreRegistry: {e}");
                    tracing::info!("Attempting fallback: re-creating engines directory");
                    let _ = std::fs::remove_dir_all(&engines_dir);
                    std::fs::create_dir_all(&engines_dir).ok();
                    crate::core_installer::CoreRegistry::new(&engines_dir)
                        .expect("CoreRegistry creation failed even after directory reset")
                }),
            );

            let github_provider: Arc<dyn crate::core_installer::ReleaseProvider> =
                Arc::new(crate::core_installer::GithubReleaseProvider::default());

            let mirror_manager = Arc::new(crate::core_installer::MirrorManager::new(vec![
                "https://github.com".into(),
            ]));

            let version_manager = Arc::new(crate::core_installer::VersionManager::new(
                core_registry.clone(),
                github_provider.clone(),
            ));

            let installer_manager = Arc::new(crate::core_installer::InstallerManager::new(
                runtime_ctx.clone(),
                core_registry.clone(),
                github_provider.clone(),
                mirror_manager,
                engines_dir,
            ));

            let update_manager = Arc::new(crate::core_installer::UpdateManager::new(
                runtime_ctx.clone(),
                version_manager.clone(),
                installer_manager.clone(),
            ));

            runtime_ctx.set_installer_manager(installer_manager);
            runtime_ctx.set_version_manager(version_manager);
            runtime_ctx.set_update_manager(update_manager.clone());

            // ===== Build Phase 14: GeoIP / GeoSite =====
            let geo_dir = app_dir_for_setup.join("data").join("geo");
            std::fs::create_dir_all(&geo_dir).ok();

            let geo_context = Arc::new(crate::geo::GeoContext::new(runtime_ctx.clone(), geo_dir));
            let geo_manager = Arc::new(crate::geo::GeoManager::new(geo_context));
            if let Err(e) = rt.block_on(async { geo_manager.initialize().await }) {
                tracing::warn!("GeoManager initialization (non-fatal): {e}");
            }
            runtime_ctx.set_geo_manager(geo_manager.clone());

            // ===== Build Phase 15: Telemetry + App Updater =====
            let telemetry_recorder = Arc::new(
                crate::telemetry::TelemetryRecorder::new(&app_dir_for_setup)
                    .expect("TelemetryRecorder creation"),
            );
            telemetry_recorder.record_startup();
            runtime_ctx.set_telemetry_recorder(telemetry_recorder.clone());

            // Start periodic telemetry flush
            {
                let tr = telemetry_recorder.clone();
                let interval = config.telemetry.sample_interval;
                std::thread::spawn(move || {
                    // We use std::thread because we need a simple interval,
                    // not a tokio task. The recorder uses parking_lot locks
                    // which are not Send-safe across await points.
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(interval));
                        tr.record_memory_sample();
                        let _ = tr.flush();
                    }
                });
            }

            // App Updater (checks for Nexus Core app updates, not engine updates)
            let app_updater = Arc::new(crate::release::AppUpdater::new(
                env!("CARGO_PKG_VERSION").to_string(),
                config.update.channel.clone(),
            ));
            runtime_ctx.set_app_updater(app_updater);

            // Start periodic geo database update checks
            if config.core.auto_update && config.geo.auto_update {
                geo_manager.start_periodic_update();
                tracing::info!(
                    "Geo auto-update enabled (interval: {}s)",
                    config.geo.update_interval
                );
            }

            // Start periodic update checks in background (if enabled)
            {
                let config = app_state_for_setup.get_config();
                if config.core.auto_update {
                    update_manager.start_periodic_check();
                    tracing::info!(
                        "Core auto-update checks enabled (interval: {}s)",
                        config.core.check_interval
                    );
                }
            }

            let core_manager = Arc::new(crate::core::CoreManager::new(runtime_ctx));
            app.manage(core_manager);

            // ===== Build system tray =====
            let tray = TrayManager::new(app).expect("Failed to build system tray");
            tray.update_connection_state("Disconnected", false);

            // ===== Start file watcher for config changes =====
            // (ConfigManager needs mutable access for start_watching;
            //  we use a clone since ConfigManager is behind Arc)
            // File watching uses PollWatcher and emits ConfigChanged events.
            // For now, registered as a background task via TaskManager.

            // Seed in-memory state from database (sync profiles/nodes/rules from repos)
            seed_memory_from_db(&app_state_for_setup, &rm);

            // ===== Start background tasks =====
            let runtime = Runtime::start(handle.clone(), (*app_state_for_setup).clone());

            // Keep runtime alive in AppState (via ResourceManager's TaskManager)
            // The tasks run until the process exits.
            std::mem::forget(runtime);

            tracing::info!("Nexus Core setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    tracing::info!("Nexus Core shutdown complete");
}

/// Seed in-memory AppState collections from database repositories.
fn seed_memory_from_db(state: &AppState, rm: &ResourceManager) {
    // Load profiles from DB into in-memory cache
    if let Ok(profiles) = rm.profile_repo.find_all() {
        if !profiles.is_empty() {
            *state.profiles.write() = profiles;
            tracing::info!(
                "Loaded {} profiles from database",
                state.profiles.read().len()
            );
        }
    }

    // Load rules from DB
    if let Ok(rules) = rm.rule_repo.find_all() {
        if !rules.is_empty() {
            *state.rules.write() = rules;
            tracing::info!("Loaded {} rules from database", state.rules.read().len());
        }
    }

    // Load settings into config
    if let Ok(settings) = rm.settings_repo.get_all() {
        let mut config = state.get_config();
        if let Some(theme) = settings.get("theme") {
            config.theme = theme.clone();
        }
        if let Some(lang) = settings.get("language") {
            config.language = lang.clone();
        }
        state.update_config(config);
    }
}

/// Initialize the tracing/logging subsystem with rolling file appenders.
///
/// Logs to:
/// - logs/app.log (INFO+, JSON format, daily rotation)
/// - logs/core.log and logs/network.log (DEBUG+, via rolling_appender module)
/// - stdout (pretty format, debug builds only)
fn init_logging(log_dir: &std::path::Path) -> Vec<tracing_appender::non_blocking::WorkerGuard> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("nexus_core=debug"));

    // Create the non-blocking rolling appender for app.log
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        log_dir,
        "app.log",
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Build the subscriber
    let subscriber = tracing_subscriber::registry().with(env_filter).with(
        fmt::layer()
            .with_writer(non_blocking)
            .with_target(true)
            .with_level(true)
            .json(),
    );

    // In debug builds, also log to stdout
    #[cfg(debug_assertions)]
    let subscriber = subscriber.with(fmt::layer().with_target(true).with_level(true).pretty());

    subscriber.init();

    tracing::info!("Logging initialized at {:?}", log_dir);
    vec![guard]
}
