use serde::Serialize;

use crate::error::map_response_error;
use crate::models::user::User;
use crate::{ModrinthClient, ModrinthError, Project};

impl ModrinthClient {
    /// Get a user by ID or username
    ///
    /// This calls:
    /// `GET /user/{id_or_username}`
    ///
    /// From <https://docs.modrinth.com/api/operations/getuser/>
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
    /// let user = client.get_user("modmuss50").await?;
    /// assert!(user.id.is_empty() == false);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user<S: Into<String>>(
        &self,
        id_or_username: S,
    ) -> Result<User, ModrinthError> {
        let url = format!("{}/user/{}", self.base_url, id_or_username.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let project = resp
            .json::<User>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(project);
    }

    /// Bulk get users
    ///
    /// This calls:
    /// `GET /users`
    ///
    /// From <https://docs.modrinth.com/api/operations/getusers/>
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    /// - [`ModrinthError::URLSerializationError`] or [`ModrinthError::JSONSerializationError`] if the URL query is unable to be constructed. (very unlikely)
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::ModrinthClient;
    /// # use modrinth_wrapped::ModrinthError;
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let users = client.get_users(vec!["modmuss50", "jellysquid3"]).await?;
    /// assert!(users.len() > 0);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_users<S: Into<String>>(
        &self,
        ids_or_usernames: Vec<S>,
    ) -> Result<Vec<User>, ModrinthError> {
        let ids_or_usernames: Vec<String> = ids_or_usernames.into_iter().map(Into::into).collect();
        let ids_json = serde_json::to_string(&ids_or_usernames)
            .map_err(ModrinthError::JSONSerializationError)?;

        #[derive(Serialize)]
        struct Params {
            ids: String,
        }

        let query = serde_urlencoded::to_string(&Params { ids: ids_json })
            .map_err(ModrinthError::URLSerializationError)?;

        let url = format!("{}/users?{}", self.base_url, query);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let ysers = resp
            .json::<Vec<User>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(ysers);
    }

    /// Get a user's projects
    ///
    /// This calls:
    /// `GET /user/{id_or_username}/projects`
    ///
    /// From <https://docs.modrinth.com/api/operations/getuserprojects/>
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
    /// let projects = client.get_user_projects("modmuss50").await?;
    /// assert!(projects.len() > 0);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_projects<S: Into<String>>(
        &self,
        id_or_username: S,
    ) -> Result<Vec<Project>, ModrinthError> {
        let url = format!("{}/user/{}/projects", self.base_url, id_or_username.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let project = resp
            .json::<Vec<Project>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(project);
    }
}
