use crate::error::map_response_error;
use crate::models::statistics::ModrinthStatistics;
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// Various statistics about this Modrinth instance.
    ///
    /// This calls:
    /// `GET /statistics`
    ///
    /// From <https://docs.modrinth.com/api/operations/statistics/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    pub async fn instance_statistics(&self) -> Result<ModrinthStatistics, ModrinthError> {
        let url = format!("{}/statistics", self.base_url);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let statistics = resp
            .json::<ModrinthStatistics>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(statistics);
    }
}
