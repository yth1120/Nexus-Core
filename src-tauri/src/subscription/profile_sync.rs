use crate::models::Profile;
use crate::utils::AppResult;

use super::subscription_downloader::SubscriptionDownloader;
use super::update_result::UpdateResult;

pub struct ProfileSync;

impl ProfileSync {
    pub async fn sync(
        profile: &Profile,
        downloader: &SubscriptionDownloader,
    ) -> AppResult<UpdateResult> {
        let url = profile.config_url.as_deref().unwrap_or("");
        let result = downloader.download(url, None, None).await?;
        tracing::info!("ProfileSync: {} -> {}", profile.name, result.status);
        Ok(result)
    }
}
