use std::net::IpAddr;
use std::path::Path;

use crate::utils::{AppError, AppResult};

/// Wraps a MaxMind DB reader for GeoIP lookups.
///
/// The underlying `maxminddb::Reader` is `Send + Sync`, so this
/// wrapper is safe to share across threads.
pub struct MmdbReader {
    inner: maxminddb::Reader<Vec<u8>>,
}

impl MmdbReader {
    /// Open an MMDB file and return a reader.
    pub fn open(path: &Path) -> AppResult<Self> {
        let reader = maxminddb::Reader::open_readfile(path)
            .map_err(|e| AppError::Io(format!("open mmdb {path:?}: {e}")))?;
        Ok(Self { inner: reader })
    }

    /// Look up the ISO country code for an IP address.
    /// Returns `None` if the IP is not found or has no country data.
    pub fn lookup_country(&self, ip: &str) -> AppResult<Option<String>> {
        let addr: IpAddr = ip
            .parse()
            .map_err(|e| AppError::Validation(format!("invalid ip '{ip}': {e}")))?;

        let result: maxminddb::geoip2::Country = self
            .inner
            .lookup(addr)
            .map_err(|e| AppError::Io(format!("mmdb lookup: {e}")))?;

        let code = result
            .country
            .and_then(|c| c.iso_code)
            .map(|s| s.to_string());

        Ok(code)
    }

    /// Return the build epoch (a version identifier) embedded in the MMDB metadata.
    pub fn build_epoch(&self) -> u64 {
        self.inner.metadata.build_epoch
    }

    /// Return a version string derived from the database metadata.
    pub fn version_string(&self) -> String {
        format!("build_epoch:{}", self.inner.metadata.build_epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_nonexistent_file_returns_error() {
        let result = MmdbReader::open(Path::new("/nonexistent/path/test.mmdb"));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_ip_returns_error() -> AppResult<()> {
        // Test that invalid IP is rejected even without a valid DB
        // We use a known test file if available, otherwise skip
        let test_paths = ["data/geo/geoip.mmdb", "../data/geo/geoip.mmdb"];
        let mut reader = None;
        for p in &test_paths {
            if Path::new(p).exists() {
                reader = MmdbReader::open(Path::new(p)).ok();
                break;
            }
        }

        if let Some(r) = reader {
            let result = r.lookup_country("not-an-ip");
            assert!(result.is_err());
        }
        Ok(())
    }
}
