use crate::config::AppConfig;
use crate::utils::AppResult;

pub struct ConfigMigration;

impl ConfigMigration {
    pub fn migrate(config: AppConfig, from: u32, to: u32) -> AppResult<AppConfig> {
        let mut c = config;
        if from < 2 && to >= 2 {
            if c.tun_stack.is_empty() {
                c.tun_stack = "system".into();
            }
            if c.traffic_mode.is_empty() {
                c.traffic_mode = "system_proxy".into();
            }
        }
        if from < 3 && to >= 3 {
            // dns_enabled, rules_enabled added in Phase 9 — defaults already applied
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn migrate_v1_to_v3() -> AppResult<()> {
        let c1 = AppConfig::default();
        let c3 = ConfigMigration::migrate(c1, 1, 3)?;
        assert_eq!(c3.tun_stack, "system");
        assert_eq!(c3.traffic_mode, "system_proxy");
        Ok(())
    }
}
