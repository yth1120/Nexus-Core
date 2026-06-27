use std::sync::Arc;

use super::geosite_reader::GeoSiteReader;

/// Matches GEOSITE rules: checks whether a domain belongs to a given category.
pub struct GeoSiteMatcher {
    reader: Arc<GeoSiteReader>,
}

impl GeoSiteMatcher {
    pub fn new(reader: Arc<GeoSiteReader>) -> Self {
        Self { reader }
    }

    /// Check whether `domain` is in the geosite category `category`.
    pub fn is_category(&self, domain: &str, category: &str) -> bool {
        self.reader.match_domain(domain, category)
    }

    /// Return all available category names.
    pub fn categories(&self) -> Vec<String> {
        self.reader.categories()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn create_test_geosite(path: &Path) -> crate::utils::AppResult<()> {
        use prost::Message;
        let site_list = crate::proto::GeoSiteList {
            entry: vec![crate::proto::GeoSite {
                tag: "github".to_string(),
                domain: vec![crate::proto::Domain {
                    r#type: "domain".to_string(),
                    value: "github.com".to_string(),
                }],
            }],
        };
        let mut buf = Vec::new();
        site_list
            .encode(&mut buf)
            .map_err(|e| crate::utils::AppError::Io(format!("encode: {e}")))?;
        std::fs::write(path, &buf)
            .map_err(|e| crate::utils::AppError::Io(format!("write: {e}")))?;
        Ok(())
    }

    #[test]
    fn geosite_matcher_works() -> crate::utils::AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("gsm-{}", uuid::Uuid::new_v4()));
        let dat_path = tmp.join("test.dat");
        create_test_geosite(&dat_path)?;
        let reader = Arc::new(GeoSiteReader::open(&dat_path)?);
        let matcher = GeoSiteMatcher::new(reader);

        assert!(matcher.is_category("api.github.com", "github"));
        assert!(!matcher.is_category("gitlab.com", "github"));

        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
