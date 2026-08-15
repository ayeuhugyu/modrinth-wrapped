use crate::error::map_response_error;
use crate::{ModrinthClient, ModrinthError, SearchQuery, SearchResult};

impl ModrinthClient {
    /// Search modrinth projects.
    ///
    /// This calls:
    /// `GET /search`
    ///
    /// From <https://docs.modrinth.com/api/operations/searchprojects/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    /// - [`ModrinthError::URLSerializationError`] if building the algorithm query fails (very unlikely)
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, SearchQuery, Facet, FacetField, FacetOperation, Sort};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let query = SearchQuery::new().with_query("Alex's Mobs");
    ///
    /// let results = client.search(&query).await?;
    /// assert!(results.total_hits > 0);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult, ModrinthError> {
        let query_parameters = query.to_url_query()?;
        let url = format!("{}/search?{}", self.base_url, query_parameters);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let results = resp
            .json::<SearchResult>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(results);
    }
}
