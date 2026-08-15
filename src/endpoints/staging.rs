use crate::error::map_response_error;
use crate::models::modrinth_build_info::ModrinthStagingInfo;
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// Get modrinth build info.
    ///
    /// This calls:
    /// `GET /`
    /// using the modrinth_staging_info url.
    ///
    /// From <https://docs.modrinth.com/api/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn modrinth_staging_info(&self) -> Result<ModrinthStagingInfo, ModrinthError> {
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&self.staging_url).send().await).await?;

        let categories = resp
            .json::<ModrinthStagingInfo>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(categories);
    }
}
