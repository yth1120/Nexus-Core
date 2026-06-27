use std::sync::Arc;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::ruleset_cache::RuleSetInfo;
use super::ruleset_compiler::RuleSetCompiler;
use super::ruleset_context::RuleSetContext;
use super::ruleset_downloader::RuleSetDownloader;
use super::ruleset_state::{RuleSetState, RuleSetStateCell};

pub struct RuleSetManager {
    #[allow(dead_code)]
    context: Arc<RuleSetContext>,
    state: RuleSetStateCell,
}

impl RuleSetManager {
    pub fn new(context: Arc<RuleSetContext>) -> Self {
        Self {
            context,
            state: RuleSetStateCell::new(),
        }
    }
    pub fn status(&self) -> RuleSetState {
        self.state.get()
    }
    pub async fn start(&self) -> AppResult<()> {
        self.state.set(RuleSetState::Running);
        Ok(())
    }
    pub async fn stop(&self) -> AppResult<()> {
        self.state.set(RuleSetState::Stopped);
        Ok(())
    }

    pub async fn download(&self, url: &str) -> AppResult<()> {
        let downloader = RuleSetDownloader;
        let raw = downloader.download(url, None).await?;
        let compiled = RuleSetCompiler::compile(&raw, "text")?;
        let count = compiled.len();
        let id = url.to_string();
        self.context.cache.insert(&id, compiled);
        self.context.runtime.publish(AppEvent::RuleSetCompiled {
            id: id.clone(),
            count,
        });
        self.context
            .runtime
            .publish(AppEvent::RuleSetDownloaded { id });
        Ok(())
    }

    pub async fn reload_all(&self) -> AppResult<()> {
        Ok(())
    }
    pub fn list(&self) -> Vec<RuleSetInfo> {
        self.context.cache.list()
    }
}
