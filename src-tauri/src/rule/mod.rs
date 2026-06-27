// Rule management layer.
//
// Phase 3: loads/reloads rules from the AppState cache into a placeholder
// cache; `compile` is a stub. Real rule compilation and matching arrive in
// Phase 4.

pub mod rule_manager;

pub use rule_manager::RuleManager;
