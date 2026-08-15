use serde::Serialize;

use crate::error::map_response_error;
use crate::{ModrinthClient, ModrinthError, Version, VersionQuery};

impl ModrinthClient {
    /// Fetch a single version by ID or slug.
    ///
    /// This calls:
    /// `GET /version/{id}`
    ///
    /// From <https://docs.modrinth.com/api/operations/getversion/>
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
    /// let version_id = project.versions.get(0).unwrap().clone();
    /// let version = client.get_version(version_id).await?;
    /// assert!(version.id.is_empty() == false);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_version<S: Into<String>>(&self, id: S) -> Result<Version, ModrinthError> {
        let url = format!("{}/version/{}", self.base_url, id.into());
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let version = resp
            .json::<Version>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(version);
    }

    /// Bulk get versions.
    ///
    /// This calls:
    /// `GET /versions`
    ///
    /// From <https://docs.modrinth.com/api/operations/getversions/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] if reqwest is unable to send/parse the request/response.
    /// - [`ModrinthError::URLSerializationError`] or [`ModrinthError::JSONSerializationError`] if the URL query is unable to be constructed. (very unlikely)
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let versions = client.get_versions(vec!["mLQ69WCw", "hu6gukgT"]).await?;
    /// assert!(versions.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_versions<S: Into<String>>(
        &self,
        ids: Vec<S>,
    ) -> Result<Vec<Version>, ModrinthError> {
        let ids: Vec<String> = ids.into_iter().map(Into::into).collect();
        let ids_json =
            serde_json::to_string(&ids).map_err(ModrinthError::JSONSerializationError)?;

        #[derive(Serialize)]
        struct Params {
            ids: String,
        }

        let query = serde_urlencoded::to_string(&Params { ids: ids_json })
            .map_err(ModrinthError::URLSerializationError)?;

        let url = format!("{}/versions?{}", self.base_url, query);
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let versions = resp
            .json::<Vec<Version>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(versions);
    }

    /// List a project's versions.
    ///
    /// This calls:
    /// `GET /project/{id_or_slug}/version`
    ///
    /// From <https://docs.modrinth.com/api/operations/getprojectversions/>
    ///
    /// JSON body contains *only* non-empty filters from `VersionQuery`:
    /// - `loaders` (optional)
    /// - `game_versions` (optional)
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    /// - [`ModrinthError::URLSerializationError`] or [`ModrinthError::JSONSerializationError`] if the URL query is unable to be constructed. (very unlikely)
    ///
    /// # Examples
    /// Without a query
    /// ```
    /// # use modrinth_wrapped::ModrinthClient;
    /// # use modrinth_wrapped::ModrinthError;
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let versions = client
    ///     .list_versions_for("fabric-api", None)
    ///     .await?;
    /// assert!(versions.len() > 0);
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// With filters
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionQuery};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let query = VersionQuery::new()
    ///     .with_game_versions(vec![String::from("1.19.1")])
    ///     .with_featured(Some(true));
    ///
    /// let versions = client
    ///     .list_versions_for("fabric-api", Some(&query))
    ///     .await?;
    ///
    /// assert!(versions.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_versions_for<S: Into<String>>(
        &self,
        id_or_slug: S,
        query: Option<&VersionQuery>,
    ) -> Result<Vec<Version>, ModrinthError> {
        let query_parameters = query.map_or(Ok(String::from("")), |q| q.to_url_query())?;
        let url = format!(
            "{}/project/{}/version?{}",
            self.base_url,
            id_or_slug.into(),
            query_parameters
        );
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let projects = resp
            .json::<Vec<Version>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(projects);
    }

    /// Get a version of a specific project using its version number or ID.
    ///
    /// This calls:
    /// `GET /project/{project_id_or_slug}/version/{version_id_or_number}`
    ///
    /// From <https://docs.modrinth.com/api/operations/getversionfromidornumber/>
    ///
    /// # Errors
    /// - [`ModrinthError::ApiError`] for any API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    ///
    /// # Examples
    /// Fetch by project slug and version ID:
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let version = client
    ///     .get_project_version_via_number("fabric-api", "mLQ69WCw")
    ///     .await?;
    ///
    /// assert!(version.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Fetch by project slug and version number:
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let version = client
    ///     .get_project_version_via_number("fabric-api", "0.143.4+26.1")
    ///     .await?;
    ///
    /// assert!(version.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_project_version_via_number<S: Into<String>, S2: Into<String>>(
        &self,
        project_id_or_slug: S,
        version_id_or_number: S2,
    ) -> Result<Version, ModrinthError> {
        let url = format!(
            "{}/project/{}/version/{}",
            self.base_url,
            project_id_or_slug.into(),
            version_id_or_number.into()
        );
        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        let projects = resp
            .json::<Version>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        return Ok(projects);
    }
}
