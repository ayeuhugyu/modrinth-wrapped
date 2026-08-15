use crate::error::map_response_error;
use crate::models::team_members::TeamMember;
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// Fetch a project's team members
    ///
    /// This calls:
    /// `GET /project/{id_or_slug}/members`
    ///
    /// From <https://docs.modrinth.com/api/operations/getprojectteammembers/>
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::ModrinthClient;
    /// # use modrinth_wrapped::ModrinthError;
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let members = client.get_project_team_members("fabric-api").await?;
    /// assert!(members.len() > 0);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_project_team_members<S: Into<String>>(
        &self,
        id_or_slug: S,
    ) -> Result<Vec<TeamMember>, ModrinthError> {
        let url = format!("{}/project/{}/members", self.base_url, id_or_slug.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let members = resp
            .json::<Vec<TeamMember>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(members);
    }

    // note: despite the lack of documentation, the `/teams?ids=["teamID1", "teamID2"]` endpoint for fetching multiple teams' members in bulk requires authorization headers
}
