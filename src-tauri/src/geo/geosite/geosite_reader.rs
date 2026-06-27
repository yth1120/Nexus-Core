use std::collections::HashMap;
use std::path::Path;

use prost::Message;

use crate::utils::{AppError, AppResult};

/// A single domain entry parsed from the protobuf database.
#[derive(Debug, Clone)]
pub struct DomainEntry {
    /// Matching type: "domain", "full", or "regex".
    pub domain_type: String,
    /// The domain value (e.g. "google.com", "*.google.com").
    pub value: String,
}

/// Reads v2fly-style geosite protobuf (.dat) files.
///
/// On construction, the file is fully parsed into a `HashMap<String, Vec<DomainEntry>>`
/// mapping category tags to their domain lists.
pub struct GeoSiteReader {
    entries: HashMap<String, Vec<DomainEntry>>,
}

impl GeoSiteReader {
    /// Open and parse a geosite protobuf file.
    pub fn open(path: &Path) -> AppResult<Self> {
        let data = std::fs::read(path).map_err(|e| AppError::Io(format!("read geosite: {e}")))?;

        // Decode using prost-generated types
        let list = crate::proto::GeoSiteList::decode(&data[..])
            .map_err(|e| AppError::Io(format!("decode geosite protobuf: {e}")))?;

        let mut entries: HashMap<String, Vec<DomainEntry>> = HashMap::new();
        for site in list.entry {
            let domains: Vec<DomainEntry> = site
                .domain
                .into_iter()
                .map(|d| DomainEntry {
                    domain_type: d.r#type,
                    value: d.value,
                })
                .collect();
            entries.insert(site.tag.to_lowercase(), domains);
        }

        tracing::info!(
            "Loaded {} geosite categories from {:?}",
            entries.len(),
            path
        );
        Ok(Self { entries })
    }

    /// Check whether `domain` belongs to the given `category`.
    ///
    /// Matching rules (matching v2fly semantics):
    /// - `"domain"` type: suffix match (e.g. `"google.com"` matches `"www.google.com"`)
    /// - `"full"` type: exact match (e.g. `"google.com"` matches only `"google.com"`)
    /// - `"regex"` type: regex match (currently returns false — regex support is future work)
    pub fn match_domain(&self, domain: &str, category: &str) -> bool {
        let cat = category.to_lowercase();
        if let Some(entries) = self.entries.get(&cat) {
            let domain_lower = domain.to_lowercase();
            for entry in entries {
                match entry.domain_type.as_str() {
                    "domain" => {
                        if domain_lower.ends_with(&format!(".{}", entry.value))
                            || domain_lower == entry.value
                        {
                            return true;
                        }
                    }
                    "full" if domain_lower == entry.value => {
                        return true;
                    }
                    _ => {
                        // "regex" and other types — not yet supported
                    }
                }
            }
        }
        false
    }

    /// Return all category tag names.
    pub fn categories(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    /// Return the number of loaded categories.
    pub fn category_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal geosite protobuf file for testing.
    fn create_test_geosite(path: &Path) -> AppResult<()> {
        use prost::Message;

        let site_list = crate::proto::GeoSiteList {
            entry: vec![
                crate::proto::GeoSite {
                    tag: "google".to_string(),
                    domain: vec![
                        crate::proto::Domain {
                            r#type: "domain".to_string(),
                            value: "google.com".to_string(),
                        },
                        crate::proto::Domain {
                            r#type: "full".to_string(),
                            value: "youtube.com".to_string(),
                        },
                    ],
                },
                crate::proto::GeoSite {
                    tag: "telegram".to_string(),
                    domain: vec![crate::proto::Domain {
                        r#type: "domain".to_string(),
                        value: "telegram.org".to_string(),
                    }],
                },
            ],
        };

        let mut buf = Vec::new();
        site_list
            .encode(&mut buf)
            .map_err(|e| AppError::Io(format!("encode: {e}")))?;
        std::fs::write(path, &buf).map_err(|e| AppError::Io(format!("write: {e}")))?;
        Ok(())
    }

    #[test]
    fn open_and_read_categories() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("gs-{}", uuid::Uuid::new_v4()));
        let dat_path = tmp.join("test.dat");
        create_test_geosite(&dat_path)?;

        let reader = GeoSiteReader::open(&dat_path)?;
        assert_eq!(reader.category_count(), 2);
        assert!(reader.categories().contains(&"google".to_string()));
        assert!(reader.categories().contains(&"telegram".to_string()));

        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn match_domain_suffix() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("gs-match-{}", uuid::Uuid::new_v4()));
        let dat_path = tmp.join("test.dat");
        create_test_geosite(&dat_path)?;

        let reader = GeoSiteReader::open(&dat_path)?;

        // Suffix match: "www.google.com" ends with ".google.com"
        assert!(reader.match_domain("www.google.com", "google"));
        // Exact match: root domain
        assert!(reader.match_domain("google.com", "google"));
        // Full match
        assert!(reader.match_domain("youtube.com", "google"));
        // No match
        assert!(!reader.match_domain("example.com", "google"));
        // Telegram
        assert!(reader.match_domain("web.telegram.org", "telegram"));
        // Unknown category
        assert!(!reader.match_domain("netflix.com", "netflix"));

        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
