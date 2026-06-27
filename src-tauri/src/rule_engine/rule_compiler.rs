use super::rule_result::RuleResult;
use crate::models::Rule;
use crate::utils::AppResult;

#[derive(Debug, Clone)]
pub struct CompiledRule {
    pub rule_type: String,
    pub payload: String,
    pub result: RuleResult,
    pub enabled: bool,
    pub rule_id: String,
}

impl CompiledRule {
    pub fn new(rule_type: &str, payload: &str, result: RuleResult) -> Self {
        Self {
            rule_type: rule_type.to_string(),
            payload: payload.to_string(),
            result,
            enabled: true,
            rule_id: String::new(),
        }
    }
}

pub fn compile(rules: &[Rule]) -> AppResult<Vec<CompiledRule>> {
    let compiled: Vec<CompiledRule> = rules
        .iter()
        .filter(|r| r.enabled)
        .map(|r| {
            let result = match r.proxy.as_str() {
                "DIRECT" => RuleResult::Direct,
                "REJECT" => RuleResult::Reject,
                _ => RuleResult::Proxy,
            };
            CompiledRule {
                rule_type: r.rule_type.clone(),
                payload: r.payload.clone(),
                result,
                enabled: r.enabled,
                rule_id: r.id.clone(),
            }
        })
        .collect();
    Ok(compiled)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compiles_rules() -> AppResult<()> {
        let rules = vec![Rule {
            id: "r1".into(),
            name: "a".into(),
            rule_type: "DomainSuffix".into(),
            payload: ".com".into(),
            proxy: "DIRECT".into(),
            enabled: true,
            tags: vec![],
            created_at: 0,
        }];
        let c = compile(&rules)?;
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].result, RuleResult::Direct);
        Ok(())
    }
}
