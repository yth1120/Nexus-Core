use std::sync::Arc;

use crate::event::AppEvent;
use crate::models::Rule;
use crate::utils::AppResult;

use super::matcher::matcher_for;
use super::rule_cache::RuleCache;
use super::rule_compiler::compile;
use super::rule_context::RuleContext;
use super::rule_matcher::RuleMatcher;
use super::rule_result::RuleResult;

type MatcherBox = Box<dyn RuleMatcher>;

pub struct RuleEngineManager {
    context: Arc<RuleContext>,
    cache: Arc<RuleCache>,
    /// Pre-constructed matchers keyed by rule_type (stateless, reusable).
    matchers: parking_lot::RwLock<std::collections::HashMap<String, Option<MatcherBox>>>,
}

impl RuleEngineManager {
    pub fn new(context: Arc<RuleContext>) -> Self {
        let cache = context.cache.clone();
        Self {
            context,
            cache,
            matchers: parking_lot::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn load(&self) -> AppResult<()> {
        let rules = self.context.runtime.app_state().rules.read().clone();
        self.compile_inner(&rules)?;
        tracing::info!("RuleEngine loaded {} rules", self.cache.len());
        Ok(())
    }

    pub async fn reload(&self) -> AppResult<()> {
        self.load().await?;
        let count = self.cache.len();
        self.context
            .runtime
            .publish(AppEvent::RuleCompiled { count });
        Ok(())
    }

    pub fn compile(&self) -> AppResult<()> {
        let rules = self.context.runtime.app_state().rules.read().clone();
        self.compile_inner(&rules)?;
        let count = self.cache.len();
        self.context
            .runtime
            .publish(AppEvent::RuleCompiled { count });
        Ok(())
    }

    fn compile_inner(&self, rules: &[Rule]) -> AppResult<()> {
        let compiled = compile(rules)?;
        self.cache.load(compiled);
        // Clear cached matchers so they are re-created with the new rule set
        self.matchers.write().clear();
        Ok(())
    }

    /// Match a connection against all compiled rules.
    ///
    /// `host` is the target domain/hostname, `port` is the target port,
    /// and `ip` is the optional source IP (needed for GEOIP rules).
    pub fn match_connection(
        &self,
        host: &str,
        _port: u16,
        ip: Option<&str>,
    ) -> AppResult<RuleResult> {
        let types = self.cache.all_types();
        for rule_type in &types {
            // Get rules by type — cheap Arc clone, no per-call allocation
            let rules = self.cache.get(rule_type);
            for cr in rules.iter() {
                if !cr.enabled {
                    continue;
                }

                // --- Geo matchers (delegated to GeoManager) ---
                if cr.rule_type == "GEOIP" || cr.rule_type == "GEOSITE" {
                    if let Some(ref geo) = self.context.runtime.get_geo_manager() {
                        match geo.match_rule(&cr.rule_type, &cr.payload, host, ip) {
                            Ok(Some(result)) => {
                                self.context.runtime.publish(AppEvent::GeoMatched {
                                    rule_type: cr.rule_type.clone(),
                                    payload: cr.payload.clone(),
                                    host: host.to_string(),
                                });
                                self.context.runtime.publish(AppEvent::RuleMatched {
                                    domain: host.to_string(),
                                    result: format!("{:?}", result),
                                });
                                return Ok(result);
                            }
                            Ok(None) => {} // no match, continue
                            Err(e) => {
                                tracing::warn!("Geo match error: {e}");
                            }
                        }
                    }
                    continue;
                }

                // --- Standard string-based matchers (cached per type) ---
                if !self.matchers.read().contains_key(&cr.rule_type) {
                    let matcher: Option<MatcherBox> = matcher_for(&cr.rule_type);
                    self.matchers.write().insert(cr.rule_type.clone(), matcher);
                }
                let guard = self.matchers.read();
                if let Some(Some(ref matcher)) = guard.get(&cr.rule_type) {
                    if matcher.match_rule(&cr.payload, host) {
                        drop(guard);
                        self.context.runtime.publish(AppEvent::RuleMatched {
                            domain: host.to_string(),
                            result: format!("{:?}", cr.result),
                        });
                        return Ok(cr.result);
                    }
                }
            }
        }
        Ok(RuleResult::Direct)
    }

    pub fn rule_count(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn load_and_match() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        // Seed a rule
        rt.app_state().rules.write().push(Rule {
            id: "r1".into(),
            name: "test".into(),
            rule_type: "DomainSuffix".into(),
            payload: ".com".into(),
            proxy: "Proxy".into(),
            enabled: true,
            tags: vec![],
            created_at: 0,
        });
        let cache = Arc::new(RuleCache::new());
        let ctx = Arc::new(RuleContext::new(rt, cache));
        let mgr = RuleEngineManager::new(ctx);
        mgr.load().await?;
        let result = mgr.match_connection("example.com", 443, None)?;
        assert_eq!(result, RuleResult::Proxy);
        Ok(())
    }
}
