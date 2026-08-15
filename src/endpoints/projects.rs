use serde::Serialize;

use crate::error::map_response_error;
use crate::models::dependency_info::DependencyInfo;
use crate::models::project::Project;
use crate::models::verify_response::VerifyResponse;
use crate::{ModrinthClient, ModrinthError};

impl ModrinthClient {
    /// Fetch a single project by ID or slug.
    ///
    /// This calls:
    /// `GET /project/{id_or_slug}`
    ///
    /// From <https://docs.modrinth.com/api/operations/getproject/>
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
    /// let project = client.get_project("fabric-api").await?;
    /// assert!(project.id.is_empty() == false);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_project<S: Into<String>>(
        &self,
        id_or_slug: S,
    ) -> Result<Project, ModrinthError> {
        let url = format!("{}/project/{}", self.base_url, id_or_slug.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let project = resp
            .json::<Project>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(project);
    }

    /// Bulk get projects.
    ///
    /// This calls:
    /// `GET /projects`
    ///
    /// From <https://docs.modrinth.com/api/operations/getprojects/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    /// - [`ModrinthError::URLSerializationError`] or [`ModrinthError::JSONSerializationError`] if the URL query is unable to be constructed. (very unlikely)
    ///
    /// A 404 not found response is **not** raised for individual missing projects.
    /// Instead, missing projects are simply omitted from the returned [`Vec`].
    ///
    ///  # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let projects = client
    ///     .get_projects(vec!["fabric-api", "sodium"])
    ///     .await?;
    ///
    /// assert!(projects.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_projects<S: Into<String>>(
        &self,
        ids_or_slugs: Vec<S>,
    ) -> Result<Vec<Project>, ModrinthError> {
        let ids_or_slugs: Vec<String> = ids_or_slugs.into_iter().map(Into::into).collect();
        let ids_json =
            serde_json::to_string(&ids_or_slugs).map_err(ModrinthError::JSONSerializationError)?;

        #[derive(Serialize)]
        struct Params {
            ids: String,
        }

        let query = serde_urlencoded::to_string(&Params { ids: ids_json })
            .map_err(ModrinthError::URLSerializationError)?;

        let url = format!("{}/projects?{}", self.base_url, query);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let projects = resp
            .json::<Vec<Project>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(projects);
    }

    /// Get the specified `count` of random projects.
    ///
    /// This calls:
    /// `GET /projects_random`
    ///
    /// From <https://docs.modrinth.com/api/operations/randomprojects/>
    ///  
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let projects = client.get_random_projects(10).await?;
    /// assert_eq!(projects.len(), 10);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_random_projects(&self, count: u8) -> Result<Vec<Project>, ModrinthError> {
        let url = format!("{}/projects_random?count={}", self.base_url, count);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let projects = resp
            .json::<Vec<Project>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(projects);
    }

    /// Check for the existence of a project via its slug or ID.
    ///
    /// Can also be used to convert slugs into IDs.
    ///
    /// This calls:
    /// `/project/{id_or_slug}/check`
    ///
    /// From <https://docs.modrinth.com/api/operations/checkprojectvalidity/>
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let verify = client.verify_project("fabric-api").await?;
    /// assert!(verify.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_project<S: Into<String>>(
        &self,
        id_or_slug: S,
    ) -> Result<VerifyResponse, ModrinthError> {
        let url = format!("{}/project/{}/check", self.base_url, id_or_slug.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let res = resp
            .json::<VerifyResponse>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(res);
    }

    /// Get a project's dependencies.
    ///
    /// This calls:
    /// `GET /project/{id_or_slug}/dependencies`
    ///
    /// From <https://docs.modrinth.com/api/operations/getdependencies/>
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let deps = client.get_dependencies_for("fabric-api").await?;
    /// assert!(deps.projects.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dependencies_for<S: Into<String>>(
        &self,
        id_or_slug: S,
    ) -> Result<DependencyInfo, ModrinthError> {
        let url = format!(
            "{}/project/{}/dependencies",
            self.base_url,
            id_or_slug.into()
        );
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let deps = resp
            .json::<DependencyInfo>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(deps);
    }
}
