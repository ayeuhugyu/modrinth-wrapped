use std::collections::HashMap;

use serde::Serialize;

use crate::error::map_response_error;
use crate::models::project::LoaderField;
use crate::models::version::VersionType;
use crate::models::version_file::VersionHashAlgorithm;
use crate::{ModrinthClient, ModrinthError, Project, Version, VersionHashQuery};

impl ModrinthClient {
    /// Fetch a single version by its file hash.
    ///
    /// This calls:
    /// `GET /version_file/{hash}`
    ///
    /// From <https://docs.modrinth.com/api/operations/versionfromhash/>
    ///
    /// # Errors
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    /// - [`ModrinthError::ReqwestError`] for request/response failures
    /// - [`ModrinthError::URLSerializationError`] if building the algorithm query fails (very unlikely)
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # use modrinth_wrapped::VersionHashAlgorithm;
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let version = client
    ///     .get_version_from_hash("3e0e0d37ea88fed51d684858a1bf24c287273e2d", VersionHashAlgorithm::Sha1)
    ///     .await?;
    /// assert!(version.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_version_from_hash<S: Into<String>>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
    ) -> Result<Version, ModrinthError> {
        #[derive(Serialize)]
        struct Query {
            algorithm: VersionHashAlgorithm,
        }

        let hash = hash.into();
        let query = serde_urlencoded::to_string(&Query { algorithm })
            .map_err(ModrinthError::URLSerializationError)?;

        let url = format!("{}/version_file/{}?{}", self.base_url, hash, query);

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.get(&url).send().await).await?;
        let version = resp
            .json::<Version>()
            .await
            .map_err(ModrinthError::ReqwestError)?;
        Ok(version)
    }

    /// Bulk get projects from version file hashes.
    ///
    /// This calls:
    /// `POST /version_file/project`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/POST/v2/version_file/project>
    ///
    /// # Errors
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::ApiError`] for any API errors
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # use modrinth_wrapped::{VersionHashAlgorithm};
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let projects = client
    ///     .get_projects_from_hashes(
    ///         vec!["3e0e0d37ea88fed51d684858a1bf24c287273e2d", "fe1389e3e68ffebdc94d5f7db41d87b81654467f"],
    ///         VersionHashAlgorithm::Sha1,
    ///     )
    ///     .await?;
    ///
    /// assert!(projects.get("3e0e0d37ea88fed51d684858a1bf24c287273e2d").unwrap().id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_projects_from_hashes<S: Into<String>>(
        &self,
        hashes: Vec<S>,
        algorithm: VersionHashAlgorithm,
    ) -> Result<HashMap<String, Project>, ModrinthError> {
        let url = format!("{}/version_file/project", self.base_url);

        #[derive(Serialize)]
        struct Params {
            algorithm: VersionHashAlgorithm,
            hashes: Vec<String>,
        }

        let hashes: Vec<String> = hashes.into_iter().map(Into::into).collect();

        let body = serde_json::to_string(&Params { algorithm, hashes })
            .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let projects = resp
            .json::<HashMap<String, Project>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(projects)
    }

    /// Bulk get versions from version file hashes.
    ///
    /// This calls:
    /// `POST /version_files`
    ///
    /// From <https://docs.modrinth.com/api/operations/versionsfromhashes/>
    ///
    /// # Errors
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::ApiError`] for any API errors
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # use modrinth_wrapped::VersionHashAlgorithm;
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let versions = client
    ///     .get_versions_from_hashes(
    ///         vec!["3e0e0d37ea88fed51d684858a1bf24c287273e2d", "fe1389e3e68ffebdc94d5f7db41d87b81654467f"],
    ///         VersionHashAlgorithm::Sha1,
    ///     )
    ///     .await?;
    /// assert!(versions.get("3e0e0d37ea88fed51d684858a1bf24c287273e2d").unwrap().id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_versions_from_hashes<S: Into<String>>(
        &self,
        hashes: Vec<S>,
        algorithm: VersionHashAlgorithm,
    ) -> Result<HashMap<String, Version>, ModrinthError> {
        let url = format!("{}/version_files", self.base_url);

        #[derive(Serialize)]
        struct Params {
            algorithm: VersionHashAlgorithm,
            hashes: Vec<String>,
        }

        let hashes: Vec<String> = hashes.into_iter().map(Into::into).collect();

        let body = serde_json::to_string(&Params { algorithm, hashes })
            .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let versions = resp
            .json::<HashMap<String, Version>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(versions)
    }

    /// Find the latest compatible version for a single hash (optionally filtered).
    ///
    /// This calls:
    /// `POST /version_file/{hash}/update`
    ///
    /// From <https://docs.modrinth.com/api/operations/getlatestversionfromhash/>
    ///
    /// JSON body contains *only* non-empty filters from `VersionHashQuery`:
    /// - `game_versions` (optional)
    /// - `loaders` (optional)
    /// - `version_types` (optional)
    ///
    /// # Errors
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for any other API errors
    ///
    /// # Examples
    /// No filters:
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError};
    /// # use modrinth_wrapped::VersionHashAlgorithm;
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let v = client
    ///     .latest_compatible_version_from_hash(
    ///         "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///         VersionHashAlgorithm::Sha1,
    ///         None,
    ///         None,
    ///     )
    ///     .await?;
    /// assert!(v.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// With filters:
    /// ```no_run
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionHashQuery};
    /// # use modrinth_wrapped::VersionType;
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let query = VersionHashQuery::new()
    ///     .with_game_versions(vec!["1.19.1"])
    ///     .with_loaders(vec!["fabric"]) // or use `Loader` enum if the feature is enabled
    ///     .with_version_types(vec![VersionType::Release]);
    ///
    /// let v = client
    ///     .latest_compatible_version_from_hash(
    ///         "c758f5dfadf35a9ab4f7f688094d599e30b2cbdb",
    ///         VersionHashAlgorithm::Sha1,
    ///         Some(&query),
    ///         None,
    ///     )
    ///     .await?;
    ///
    /// assert!(v.id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn latest_compatible_version_from_hash<S: Into<String>>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        query: Option<&VersionHashQuery>,
        version_id: Option<S>,
    ) -> Result<Version, ModrinthError> {
        #[derive(Serialize)]
        struct UpdateQuery {
            algorithm: VersionHashAlgorithm,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_id: Option<String>,
        }

        let hash = hash.into();
        let update_query = UpdateQuery {
            algorithm,
            version_id: version_id.map(Into::into),
        };

        let query_str = serde_urlencoded::to_string(&update_query)
            .map_err(ModrinthError::URLSerializationError)?;

        let url = format!(
            "{}/version_file/{}/update?{}",
            self.base_url, hash, query_str
        );

        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            game_versions: Option<&'a Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            loaders: Option<&'a Vec<LoaderField>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_types: Option<&'a Vec<VersionType>>,
        }

        let (game_versions, loaders, version_types) = query
            .map(|q| q.filters_as_optional_refs())
            .unwrap_or((None, None, None));

        let body = serde_json::to_string(&Params {
            game_versions,
            loaders,
            version_types,
        })
        .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let version = resp
            .json::<Version>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(version)
    }

    /// Find the latest compatible versions from multiple hashes (optionally filtered).
    ///
    /// This calls:
    /// `POST /version_files/update`
    ///
    /// From <https://docs.modrinth.com/api/operations/getlatestversionsfromhashes/>
    ///
    /// # Errors
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::ApiError`] for any API errors
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionHashQuery};
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let query = VersionHashQuery::new()
    ///     .with_game_versions(vec!["1.20.1"]);
    ///
    /// let versions = client
    ///     .latest_compatible_version_from_hashes(
    ///         vec!["2824689367eec2b52c6085e968343751137e66bf", "89c79de1c8e17f3d34fded3a2fe401e7e60a707d"],
    ///         VersionHashAlgorithm::Sha1,
    ///         Some(&query),
    ///     )
    ///     .await?;
    ///
    /// assert!(versions.get("2824689367eec2b52c6085e968343751137e66bf").unwrap().id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn latest_compatible_version_from_hashes<S: Into<String>>(
        &self,
        hashes: Vec<S>,
        algorithm: VersionHashAlgorithm,
        query: Option<&VersionHashQuery>,
    ) -> Result<HashMap<String, Version>, ModrinthError> {
        let url = format!("{}/version_files/update", self.base_url);

        #[derive(Serialize)]
        struct Params<'a> {
            algorithm: VersionHashAlgorithm,
            hashes: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            game_versions: Option<&'a Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            loaders: Option<&'a Vec<LoaderField>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_types: Option<&'a Vec<VersionType>>,
        }

        let hashes: Vec<String> = hashes.into_iter().map(Into::into).collect();

        let (game_versions, loaders, version_types) = query
            .map(|q| q.filters_as_optional_refs())
            .unwrap_or((None, None, None));

        let body = serde_json::to_string(&Params {
            algorithm,
            hashes,
            game_versions,
            loaders,
            version_types,
        })
        .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let version = resp
            .json::<HashMap<String, Version>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(version)
    }

    /// Get *all* latest compatible versions for multiple hashes (optionally filtered).
    ///
    /// This calls:
    /// `POST /version_files/update_many`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/POST/v2/version_files/update_individual>
    ///
    /// # Errors
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::ApiError`] for any API errors
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionHashQuery};
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let query = VersionHashQuery::new().with_game_versions(vec!["1.19.4"]);
    ///
    /// let versions = client
    ///     .all_latest_compatible_versions_from_hashes(
    ///         vec!["586ac174c831da9fd35edce1e36eb14c7f878ce3", "84f19148833a29fadac9e0405596bcca502d3a53"],
    ///         VersionHashAlgorithm::Sha1,
    ///         Some(&query),
    ///     )
    ///     .await?;
    ///
    /// assert!(versions.get("586ac174c831da9fd35edce1e36eb14c7f878ce3").unwrap().len() > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn all_latest_compatible_versions_from_hashes<S: Into<String>>(
        &self,
        hashes: Vec<S>,
        algorithm: VersionHashAlgorithm,
        query: Option<&VersionHashQuery>,
    ) -> Result<HashMap<String, Vec<Version>>, ModrinthError> {
        let url = format!("{}/version_files/update_many", self.base_url);

        #[derive(Serialize)]
        struct Params<'a> {
            algorithm: VersionHashAlgorithm,
            hashes: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            game_versions: Option<&'a Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            loaders: Option<&'a Vec<LoaderField>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_types: Option<&'a Vec<VersionType>>,
        }

        let hashes: Vec<String> = hashes.into_iter().map(Into::into).collect();

        let (game_versions, loaders, version_types) = query
            .map(|q| q.filters_as_optional_refs())
            .unwrap_or((None, None, None));

        let body = serde_json::to_string(&Params {
            algorithm,
            hashes,
            game_versions,
            loaders,
            version_types,
        })
        .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let versions = resp
            .json::<HashMap<String, Vec<Version>>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(versions)
    }

    /// Get the latest compatible versions with *per-hash* filters.
    ///
    /// This calls:
    /// `POST /version_files/update_individual`
    ///
    /// Any empty filter vectors are omitted (`game_versions`, `loaders`, `version_types`).
    ///
    /// # Errors
    /// - [`ModrinthError::MissingHash`] if any `VersionHashQuery` in `queres` has `hash == None`
    /// - [`ModrinthError::JSONSerializationError`] if the request body can't be serialized
    /// - [`ModrinthError::ReqwestError`] if the HTTP request/response fails
    /// - [`ModrinthError::ApiError`] for any API errors
    ///
    /// A 404 not found response is **not** raised for individual missing versions.
    /// Instead, missing versions are simply omitted from the returned [`Vec`].
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionHashQuery};
    /// # use modrinth_wrapped::{VersionType};
    /// #
    /// # async fn demo(client: &ModrinthClient) -> Result<(), ModrinthError> {
    /// let queries = vec![
    ///     VersionHashQuery::new()
    ///         .with_hash("2824689367eec2b52c6085e968343751137e66bf")
    ///         .with_game_versions(vec!["1.20.1"])
    ///         .with_loaders(vec!["fabric"]) // or use `Loader` enum if the feature is enabled
    ///         .with_version_types(vec![VersionType::Release]),
    ///     VersionHashQuery::new()
    ///         .with_hash("586ac174c831da9fd35edce1e36eb14c7f878ce3")
    ///         .with_game_versions(vec!["1.19.4"]),
    /// ];
    ///
    /// let versions = client
    ///     .latest_versions_with_individual_filters(VersionHashAlgorithm::Sha1, queries)
    ///     .await?;
    ///
    /// assert!(versions.get("2824689367eec2b52c6085e968343751137e66bf").unwrap().id.is_empty() == false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn latest_versions_with_individual_filters(
        &self,
        algorithm: VersionHashAlgorithm,
        queries: Vec<VersionHashQuery>,
    ) -> Result<HashMap<String, Version>, ModrinthError> {
        #[derive(Serialize)]
        struct HashEntry {
            #[serde(skip_serializing_if = "Option::is_none")]
            game_versions: Option<Vec<String>>,
            hash: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            loaders: Option<Vec<LoaderField>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_types: Option<Vec<VersionType>>,
        }

        #[derive(Serialize)]
        struct Params {
            algorithm: VersionHashAlgorithm,
            hashes: Vec<HashEntry>,
        }

        let hashes: Vec<HashEntry> = queries
            .into_iter()
            .map(|q| {
                let hash = q.hash.ok_or_else(|| ModrinthError::MissingHash)?;

                Ok(HashEntry {
                    game_versions: (!q.game_versions.is_empty()).then_some(q.game_versions),
                    hash,
                    loaders: (!q.loaders.is_empty()).then_some(q.loaders),
                    version_types: (!q.version_types.is_empty()).then_some(q.version_types),
                })
            })
            .collect::<Result<Vec<_>, ModrinthError>>()?;

        let url = format!("{}/version_files/update_individual", self.base_url);

        let body = serde_json::to_string(&Params { algorithm, hashes })
            .map_err(ModrinthError::JSONSerializationError)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;
        let resp = map_response_error(self.http_client.post(&url).body(body).send().await).await?;

        let versions = resp
            .json::<HashMap<String, Version>>()
            .await
            .map_err(ModrinthError::ReqwestError)?;

        Ok(versions)
    }
}
