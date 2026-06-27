use std::sync::Arc;

use crate::models::Rule;
use crate::repository::RuleRepository;
use crate::storage::Database;
use crate::utils::{AppError, AppResult};

pub struct SqliteRuleRepository {
    db: Arc<Database>,
}

impl SqliteRuleRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl RuleRepository for SqliteRuleRepository {
    fn find_all(&self) -> AppResult<Vec<Rule>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, rule_type, payload, proxy, enabled, tags, created_at
                 FROM rules ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                let tags_str: String = row.get(6)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(Rule {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    rule_type: row.get(2)?,
                    payload: row.get(3)?,
                    proxy: row.get(4)?,
                    enabled: row.get::<_, i32>(5)? != 0,
                    tags,
                    created_at: row.get(7)?,
                })
            })?;

            let mut rules = Vec::new();
            for row in rows {
                rules.push(row?);
            }
            Ok(rules)
        })
    }

    fn find_by_id(&self, id: &str) -> AppResult<Option<Rule>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, rule_type, payload, proxy, enabled, tags, created_at
                 FROM rules WHERE id = ?1",
            )?;
            let mut rows = stmt.query_map([id], |row| {
                let tags_str: String = row.get(6)?;
                let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                Ok(Rule {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    rule_type: row.get(2)?,
                    payload: row.get(3)?,
                    proxy: row.get(4)?,
                    enabled: row.get::<_, i32>(5)? != 0,
                    tags,
                    created_at: row.get(7)?,
                })
            })?;
            Ok(rows.next().transpose()?)
        })
    }

    fn insert(&self, rule: &Rule) -> AppResult<()> {
        let tags_json = serde_json::to_string(&rule.tags).unwrap_or_default();
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO rules (id, name, rule_type, payload, proxy, enabled, tags, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![
                    rule.id,
                    rule.name,
                    rule.rule_type,
                    rule.payload,
                    rule.proxy,
                    rule.enabled as i32,
                    tags_json,
                    rule.created_at,
                ],
            )?;
            Ok(())
        })
    }

    fn update(&self, rule: &Rule) -> AppResult<()> {
        self.insert(rule)
    }

    fn delete(&self, id: &str) -> AppResult<()> {
        self.db.with_connection(|conn| {
            let deleted = conn.execute("DELETE FROM rules WHERE id = ?1", [id])?;
            if deleted == 0 {
                return Err(AppError::NotFound(format!("Rule {}", id)));
            }
            Ok(())
        })
    }

    fn toggle_enabled(&self, id: &str) -> AppResult<Rule> {
        self.db.with_connection(|conn| {
            let enabled: i32 = conn
                .query_row("SELECT enabled FROM rules WHERE id = ?1", [id], |row| {
                    row.get(0)
                })
                .map_err(|_| AppError::NotFound(format!("Rule {}", id)))?;

            let new_enabled = if enabled != 0 { 0 } else { 1 };
            conn.execute(
                "UPDATE rules SET enabled = ?1 WHERE id = ?2",
                rusqlite::params![new_enabled, id],
            )?;

            // Return the updated rule
            self.find_by_id(id)?
                .ok_or_else(|| AppError::NotFound(format!("Rule {}", id)))
        })
    }

    fn count(&self) -> AppResult<usize> {
        self.db.with_connection(|conn| {
            let count: i64 = conn.query_row("SELECT COUNT(*) FROM rules", [], |row| row.get(0))?;
            Ok(count as usize)
        })
    }
}
