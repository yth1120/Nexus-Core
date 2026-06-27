use crate::models::{Node, Rule};

#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub status: String,
    pub nodes: Vec<Node>,
    pub rules: Vec<Rule>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}
