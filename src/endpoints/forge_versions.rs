use crate::error::map_response_error;
use crate::models::forge_updates::ForgeUpdates;
use crate::types::forge_updates_type::ForgeUpdatesQueryType;
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// Get a mod's forge updates JSON file.
    ///
    /// Note: not recommended for use. This endpoint is primarily used by the [Forge Update Checker](https://docs.minecraftforge.net/en/latest/misc/updatechecker/).
    ///
    /// This calls:
    /// `GET /updates/{id_or_slug}/forge_updates.json`
    /// using the forge_updates url.
    ///
    /// From <https://docs.modrinth.com/api/operations/forgeupdates/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// Note: this does not return a 404 response for unknown mods / non-forge mods, but instead a [`ModrinthError::ApiError`].
    pub async fn get_forge_updates<S: Into<String>>(
        &self,
        id_or_slug: S,
        forge_query_type: ForgeUpdatesQueryType,
    ) -> Result<ForgeUpdates, ModrinthError> {
        let query = match forge_query_type {
            ForgeUpdatesQueryType::Both => "?neoforge=include",
            ForgeUpdatesQueryType::NeoForgeOnly => "?neoforge=only",
            ForgeUpdatesQueryType::ForgeOnly => "",
        };
        let url = format!(
            "{}/updates/{}/forge_updates.json{}",
            self.forge_updates_base_url,
            id_or_slug.into(),
            query
        );
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let forge_updates = resp
            .json::<ForgeUpdates>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(forge_updates);
    }
}
