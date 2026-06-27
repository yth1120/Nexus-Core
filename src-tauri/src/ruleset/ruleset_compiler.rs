use crate::models::Rule;
use crate::rule_engine::CompiledRule;
use crate::utils::AppResult;

pub struct RuleSetCompiler;

impl RuleSetCompiler {
    pub fn compile(raw: &str, _format: &str) -> AppResult<Vec<CompiledRule>> {
        let mut rules = Vec::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((rule_type, rest)) = line.split_once(',') {
                let rt = rule_type.trim().to_string();
                let payload = rest.trim().to_string();
                rules.push(Rule {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("rs-{}", rules.len()),
                    rule_type: rt,
                    payload,
                    proxy: "DIRECT".into(),
                    enabled: true,
                    tags: vec![],
                    created_at: 0,
                });
            }
        }
        crate::rule_engine::compile(&rules)
    }
}
