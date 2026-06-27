use std::path::PathBuf;
use std::sync::Arc;

use crate::runtime::RuntimeContext;

use super::cache::geo_cache::GeoCache;

/// Dependency-injection container for the Geo module.
pub struct GeoContext {
    pub runtime: Arc<RuntimeContext>,
    pub geo_dir: PathBuf,
    pub geoip_path: PathBuf,
    pub geosite_path: PathBuf,
    pub cache: Arc<GeoCache>,
}

impl GeoContext {
    pub fn new(runtime: Arc<RuntimeContext>, geo_dir: PathBuf) -> Self {
        let config = runtime.app_state().get_config();
        let geoip_path = geo_dir.join(&config.geo.geoip_path);
        let geosite_path = geo_dir.join(&config.geo.geosite_path);
        let cache = Arc::new(GeoCache::new(4096, 3600));

        Self {
            runtime,
            geo_dir,
            geoip_path,
            geosite_path,
            cache,
        }
    }
}
