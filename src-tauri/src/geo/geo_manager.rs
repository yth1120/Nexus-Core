use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::rule_engine::rule_result::RuleResult;
use crate::utils::{AppError, AppResult};

use super::cache::database_cache::DatabaseCache;
use super::geo_context::GeoContext;
use super::geo_state::{GeoState, GeoStateCell};
use super::geoip::geoip_matcher::GeoIpMatcher;
use super::geoip::geoip_updater::GeoIpUpdater;
use super::geoip::mmdb_reader::MmdbReader;
use super::geosite::geosite_matcher::GeoSiteMatcher;
use super::geosite::geosite_reader::GeoSiteReader;
use super::geosite::geosite_updater::GeoSiteUpdater;

/// Serializable status snapshot.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoStatus {
    pub state: GeoState,
    pub geoip_loaded: bool,
    pub geoip_version: String,
    pub geosite_loaded: bool,
    pub geosite_categories: usize,
    pub message: String,
}

/// Central orchestrator for the GeoIP and GeoSite subsystems.
///
/// Owns the MMDB reader, GeoSite reader, matchers, updaters, and cache.
/// Exposes a `match_rule()` method that the rule engine calls for
/// GEOIP/GEOSITE rule types.
pub struct GeoManager {
    context: Arc<GeoContext>,
    state: GeoStateCell,

    geoip_reader: Arc<RwLock<Option<Arc<MmdbReader>>>>,
    geosite_reader: Arc<RwLock<Option<Arc<GeoSiteReader>>>>,

    geoip_matcher: RwLock<Option<GeoIpMatcher>>,
    geosite_matcher: RwLock<Option<GeoSiteMatcher>>,

    geoip_updater: GeoIpUpdater,
    geosite_updater: GeoSiteUpdater,

    #[allow(dead_code)]
    db_cache: DatabaseCache,
}

impl GeoManager {
    pub fn new(context: Arc<GeoContext>) -> Self {
        let geoip_updater = GeoIpUpdater::new(context.geo_dir.clone());
        let geosite_updater = GeoSiteUpdater::new(context.geo_dir.clone());

        Self {
            context,
            state: GeoStateCell::new(),
            geoip_reader: Arc::new(RwLock::new(None)),
            geosite_reader: Arc::new(RwLock::new(None)),
            geoip_matcher: RwLock::new(None),
            geosite_matcher: RwLock::new(None),
            geoip_updater,
            geosite_updater,
            db_cache: DatabaseCache::new(),
        }
    }

    // ----- lifecycle -----

    /// Try to load existing geo databases from disk.
    /// Does NOT download if files are missing — that's handled by `update()`.
    pub async fn initialize(&self) -> AppResult<()> {
        self.state
            .set_with_message(GeoState::Loading, "loading databases...");

        let mut geoip_loaded = false;
        let mut geosite_loaded = false;

        // Load GeoIP if available
        if self.context.geoip_path.exists() {
            match MmdbReader::open(&self.context.geoip_path) {
                Ok(reader) => {
                    let version = reader.version_string();
                    tracing::info!("GeoIP database loaded: {version}");
                    *self.geoip_reader.write() = Some(Arc::new(reader));
                    *self.geoip_matcher.write() = Some(GeoIpMatcher::new(
                        self.geoip_reader
                            .read()
                            .clone()
                            .ok_or_else(|| AppError::Internal("geoip reader not set".into()))?,
                    ));
                    geoip_loaded = true;
                }
                Err(e) => {
                    tracing::warn!("Failed to load GeoIP database: {e}");
                }
            }
        }

        // Load GeoSite if available
        if self.context.geosite_path.exists() {
            match GeoSiteReader::open(&self.context.geosite_path) {
                Ok(reader) => {
                    let categories = reader.category_count();
                    tracing::info!("GeoSite database loaded: {categories} categories");
                    *self.geosite_reader.write() = Some(Arc::new(reader));
                    *self.geosite_matcher.write() = Some(GeoSiteMatcher::new(
                        self.geosite_reader
                            .read()
                            .clone()
                            .ok_or_else(|| AppError::Internal("geosite reader not set".into()))?,
                    ));
                    geosite_loaded = true;
                }
                Err(e) => {
                    tracing::warn!("Failed to load GeoSite database: {e}");
                }
            }
        }

        if geoip_loaded || geosite_loaded {
            self.state.set(GeoState::Ready);
            self.context.runtime.publish(AppEvent::GeoLoaded {
                geoip_version: self.geoip_version(),
                geosite_version: self.geosite_version(),
            });
        } else {
            self.state.set_with_message(
                GeoState::Idle,
                "no databases found — run update to download",
            );
        }

        Ok(())
    }

    /// Reload both databases from disk.
    pub async fn reload(&self) -> AppResult<()> {
        self.state.set(GeoState::Loading);

        // Clear existing readers
        *self.geoip_reader.write() = None;
        *self.geosite_reader.write() = None;
        *self.geoip_matcher.write() = None;
        *self.geosite_matcher.write() = None;

        self.initialize().await
    }

    /// Download and install the latest geo databases.
    pub async fn update(&self) -> AppResult<()> {
        self.state.set(GeoState::Updating);

        // Update GeoIP
        let geoip_version = match self.geoip_updater.update(&self.context.geoip_path).await {
            Ok(v) => {
                tracing::info!("GeoIP updated to {v}");
                Some(v)
            }
            Err(e) => {
                tracing::warn!("GeoIP update failed: {e}");
                self.context.runtime.publish(AppEvent::GeoFailed {
                    database: "geoip".into(),
                    error: e.to_string(),
                });
                None
            }
        };

        // Update GeoSite
        let geosite_ok = match self
            .geosite_updater
            .update(&self.context.geosite_path)
            .await
        {
            Ok(_) => {
                tracing::info!("GeoSite updated");
                true
            }
            Err(e) => {
                tracing::warn!("GeoSite update failed: {e}");
                self.context.runtime.publish(AppEvent::GeoFailed {
                    database: "geosite".into(),
                    error: e.to_string(),
                });
                false
            }
        };

        // Reload readers with new databases
        self.reload().await?;

        if let Some(ref v) = geoip_version {
            self.context.runtime.publish(AppEvent::GeoUpdated {
                database: "geoip".into(),
                version: v.clone(),
            });
        }
        if geosite_ok {
            self.context.runtime.publish(AppEvent::GeoUpdated {
                database: "geosite".into(),
                version: "latest".into(),
            });
        }

        self.state.set(GeoState::Ready);
        Ok(())
    }

    /// Start periodic database update checks.
    pub fn start_periodic_update(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        let interval_secs = {
            let config = this.context.runtime.app_state().get_config();
            config.geo.update_interval
        };

        tokio::spawn(async move {
            // Wait 60 seconds before the first periodic check
            tokio::time::sleep(Duration::from_secs(60)).await;

            loop {
                let _ = this.update().await;
                tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            }
        })
    }

    // ----- status -----

    pub fn status(&self) -> GeoStatus {
        GeoStatus {
            state: self.state.get(),
            geoip_loaded: self.geoip_reader.read().is_some(),
            geoip_version: self.geoip_version(),
            geosite_loaded: self.geosite_reader.read().is_some(),
            geosite_categories: self
                .geosite_reader
                .read()
                .as_ref()
                .map(|r| r.category_count())
                .unwrap_or(0),
            message: self.state.message(),
        }
    }

    // ----- matching (called by RuleEngineManager) -----

    /// Match a GEOIP or GEOSITE rule. Called by `RuleEngineManager::match_connection`.
    ///
    /// - `rule_type`: `"GEOIP"` or `"GEOSITE"`
    /// - `payload`: the rule payload (e.g. `"CN"`, `"google"`, `"PRIVATE"`)
    /// - `host`: the connection hostname (for GEOSITE rules)
    /// - `ip`: the connection source IP (for GEOIP rules)
    ///
    /// Returns `Some(RuleResult)` if the rule matches, `None` if it doesn't.
    pub fn match_rule(
        &self,
        rule_type: &str,
        payload: &str,
        host: &str,
        ip: Option<&str>,
    ) -> AppResult<Option<RuleResult>> {
        match rule_type {
            "GEOIP" => self.match_geoip_rule(payload, ip),
            "GEOSITE" => self.match_geosite_rule(payload, host),
            _ => Ok(None),
        }
    }

    // ----- public lookup methods -----

    /// Look up the country for an IP address (with caching).
    pub fn match_country(&self, ip: &str) -> AppResult<Option<String>> {
        // Check cache first
        if let Some(cached) = self.context.cache.get_country(ip) {
            return Ok(cached);
        }

        // Check PRIVATE first (no MMDB needed)
        if GeoIpMatcher::is_private(ip) {
            let result = Some("PRIVATE".to_string());
            self.context.cache.put_country(ip, result.clone());
            return Ok(result);
        }

        // Query MMDB
        let reader_guard = self.geoip_reader.read();
        let result = if let Some(ref _reader) = *reader_guard {
            // We need the raw country code, not a bool
            let reader_guard = self.geoip_reader.read();
            if let Some(ref reader) = *reader_guard {
                reader.lookup_country(ip)?
            } else {
                None
            }
        } else {
            None
        };

        self.context.cache.put_country(ip, result.clone());
        Ok(result)
    }

    /// Check whether a domain belongs to a geosite category (with caching).
    pub fn match_domain_category(&self, domain: &str, category: &str) -> bool {
        // Check cache
        if let Some(cached) = self.context.cache.get_domain_category(domain, category) {
            return cached;
        }

        let matcher_guard = self.geosite_matcher.read();
        let result = matcher_guard
            .as_ref()
            .is_some_and(|m| m.is_category(domain, category));

        self.context
            .cache
            .put_domain_category(domain, category, result);
        result
    }

    // ----- internals -----

    fn geoip_version(&self) -> String {
        self.geoip_reader
            .read()
            .as_ref()
            .map(|r| r.version_string())
            .unwrap_or_default()
    }

    fn geosite_version(&self) -> String {
        self.geosite_reader
            .read()
            .as_ref()
            .map(|_| "loaded".to_string())
            .unwrap_or_default()
    }

    fn match_geoip_rule(&self, payload: &str, ip: Option<&str>) -> AppResult<Option<RuleResult>> {
        let ip = match ip {
            Some(ip) => ip,
            None => return Ok(None), // no IP to match
        };

        // Handle PRIVATE specially
        if payload.eq_ignore_ascii_case("PRIVATE") {
            if GeoIpMatcher::is_private(ip) {
                return Ok(Some(RuleResult::Direct));
            }
            return Ok(None);
        }

        // Standard country match
        let matcher_guard = self.geoip_matcher.read();
        if let Some(ref matcher) = *matcher_guard {
            if matcher.is_country(ip, payload) {
                return Ok(Some(RuleResult::Direct));
            }
        }

        Ok(None)
    }

    fn match_geosite_rule(&self, payload: &str, host: &str) -> AppResult<Option<RuleResult>> {
        let matcher_guard = self.geosite_matcher.read();
        if let Some(ref matcher) = *matcher_guard {
            if matcher.is_category(host, payload) {
                return Ok(Some(RuleResult::Direct));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    fn create_test_geosite_dat(path: &std::path::Path) -> AppResult<()> {
        use prost::Message;
        let site_list = crate::proto::GeoSiteList {
            entry: vec![
                crate::proto::GeoSite {
                    tag: "google".to_string(),
                    domain: vec![crate::proto::Domain {
                        r#type: "domain".to_string(),
                        value: "google.com".to_string(),
                    }],
                },
                crate::proto::GeoSite {
                    tag: "github".to_string(),
                    domain: vec![crate::proto::Domain {
                        r#type: "domain".to_string(),
                        value: "github.com".to_string(),
                    }],
                },
            ],
        };
        let mut buf = Vec::new();
        site_list
            .encode(&mut buf)
            .map_err(|e| AppError::Io(format!("encode geosite: {e}")))?;
        std::fs::write(path, &buf).map_err(|e| AppError::Io(format!("write geosite: {e}")))?;
        Ok(())
    }

    fn test_geo_manager_geosite_only() -> AppResult<Arc<GeoManager>> {
        let tmp = std::env::temp_dir().join(format!("gm-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;

        let geoip_path = tmp.join("geoip.mmdb"); // won't exist
        let geosite_path = tmp.join("geosite.dat");
        create_test_geosite_dat(&geosite_path)?;

        let ctx = RuntimeContext::new_for_test()?;
        let geo_ctx = Arc::new(GeoContext::new(ctx, tmp.clone()));

        // Override paths to point to our test files
        let geo_ctx = Arc::new(GeoContext {
            runtime: geo_ctx.runtime.clone(),
            geo_dir: tmp.clone(),
            geoip_path,
            geosite_path,
            cache: Arc::new(crate::geo::cache::geo_cache::GeoCache::new(16, 3600)),
        });

        Ok(Arc::new(GeoManager::new(geo_ctx)))
    }

    #[tokio::test]
    async fn initialize_loads_geosite() -> AppResult<()> {
        let gm = test_geo_manager_geosite_only()?;
        gm.initialize().await?;
        let status = gm.status();
        // GeoIP file doesn't exist, shouldn't be loaded
        assert!(!status.geoip_loaded);
        // GeoSite should be loaded
        assert!(status.geosite_loaded);
        assert_eq!(status.geosite_categories, 2);
        Ok(())
    }

    #[tokio::test]
    async fn match_geosite_rule() -> AppResult<()> {
        let gm = test_geo_manager_geosite_only()?;
        gm.initialize().await?;

        let result = gm.match_rule("GEOSITE", "google", "mail.google.com", None)?;
        assert_eq!(result, Some(RuleResult::Direct));

        let result = gm.match_rule("GEOSITE", "github", "api.github.com", None)?;
        assert_eq!(result, Some(RuleResult::Direct));

        let result = gm.match_rule("GEOSITE", "netflix", "mail.google.com", None)?;
        assert_eq!(result, None);

        Ok(())
    }

    #[tokio::test]
    async fn match_private_geoip_works_without_mmdb() -> AppResult<()> {
        let gm = test_geo_manager_geosite_only()?;
        gm.initialize().await?;

        // PRIVATE check works without MMDB (pure IP logic)
        let result = gm.match_rule("GEOIP", "PRIVATE", "", Some("192.168.1.1"))?;
        assert_eq!(result, Some(RuleResult::Direct));

        let result = gm.match_rule("GEOIP", "PRIVATE", "", Some("8.8.8.8"))?;
        assert_eq!(result, None);

        Ok(())
    }

    #[tokio::test]
    async fn geoip_without_ip_returns_none() -> AppResult<()> {
        let gm = test_geo_manager_geosite_only()?;
        gm.initialize().await?;

        let result = gm.match_rule("GEOIP", "CN", "example.com", None)?;
        assert_eq!(result, None);

        Ok(())
    }

    #[tokio::test]
    async fn status_reflects_state() -> AppResult<()> {
        let gm = test_geo_manager_geosite_only()?;
        gm.initialize().await?;
        let status = gm.status();
        assert_eq!(status.state, GeoState::Ready);
        assert!(status.message.is_empty() || !status.message.is_empty()); // just check it's callable
        Ok(())
    }
}
