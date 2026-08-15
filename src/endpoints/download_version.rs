use std::io::Write;

use reqwest::Response;
use serde::Serialize;

use crate::error::map_response_error;
use crate::models::version_file::VersionHashAlgorithm;
use crate::{DownloadProgress, ModrinthClient, ModrinthError};

impl ModrinthClient {
    fn version_file_download_url(
        &self,
        hash: &str,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
    ) -> Result<String, ModrinthError> {
        #[derive(Serialize)]
        struct Query<'a> {
            algorithm: VersionHashAlgorithm,

            #[serde(skip_serializing_if = "Option::is_none")]
            version_id: Option<&'a str>,
        }

        let query = serde_urlencoded::to_string(&Query {
            algorithm,
            version_id,
        })
        .map_err(ModrinthError::URLSerializationError)?;

        Ok(format!(
            "{}/version_file/{}/download?{}",
            self.base_url, hash, query
        ))
    }

    /// Start downloading a version file by file hash and return the raw [`reqwest::Response`].
    ///
    /// This calls:
    /// `GET /version_file/{hash}/download`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/GET/v2/version_file/{version_id}/download>
    ///
    /// This is the lowest-level download method. Use it when you want to consume the response manually.
    ///
    /// # Errors
    /// - [`ModrinthError::URLSerializationError`] if building the query string fails (very unlikely)
    /// - [`ModrinthError::ReqwestError`] if the HTTP request fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for other API errors
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let mut response = client
    ///     .download_file_from_hash_response(
    ///         "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///         VersionHashAlgorithm::Sha1,
    ///         None,
    ///     )
    ///     .await?;
    ///
    /// while let Some(chunk) = response
    ///     .chunk()
    ///     .await
    ///     .map_err(ModrinthError::ReqwestError)?
    /// {
    ///     println!("received {} bytes", chunk.len());
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file_from_hash_response<S: AsRef<str>>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
    ) -> Result<Response, ModrinthError> {
        let url = self.version_file_download_url(hash.as_ref(), algorithm, version_id)?;

        #[cfg(feature = "rate-limited")]
        let _permit = self.rate_limiter.acquire().await;

        let resp = map_response_error(self.http_client.get(&url).send().await).await?;

        Ok(resp)
    }

    /// Download a version file by file hash into memory.
    ///
    /// This calls:
    /// `GET /version_file/{hash}/download`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/GET/v2/version_file/{version_id}/download>
    ///
    /// This buffers the entire file into a [`Vec<u8>`]. For large files, or really just in general, use [`ModrinthClient::download_file_from_hash_to_writer`] to stream the data into a writer.
    ///
    /// # Errors
    /// - [`ModrinthError::URLSerializationError`] if building the query string fails (very unlikely)
    /// - [`ModrinthError::ReqwestError`] if the HTTP request or response body read fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for other API errors
    ///
    /// # Examples
    /// ```no_run
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    ///
    /// let bytes = client
    ///     .download_file_from_hash_bytes(
    ///         "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///         VersionHashAlgorithm::Sha1,
    ///         None,
    ///     )
    ///     .await?;
    ///
    /// assert!(!bytes.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file_from_hash_bytes<S: AsRef<str>>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
    ) -> Result<Vec<u8>, ModrinthError> {
        let resp = self
            .download_file_from_hash_response(hash, algorithm, version_id)
            .await?;

        let bytes = resp.bytes().await.map_err(ModrinthError::ReqwestError)?;

        Ok(bytes.to_vec())
    }

    /// Download a version file by file hash into a writer.
    ///
    /// This calls:
    /// `GET /version_file/{hash}/download`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/GET/v2/version_file/{version_id}/download>
    ///
    /// Returns the number of bytes written.
    ///
    /// If you want progress updates, use [`ModrinthClient::download_file_from_hash_to_writer_with_progress`].
    ///
    /// It is recommended to use asynchronous writes via [`ModrinthClient::download_file_from_hash_to_async_writer`] over this function, however if you cannot use Tokio, don't care, or have some other reason this function is made available. Disabling the `tokio-io` feature will disable that function and its tokio usage.
    ///
    /// # Errors
    /// - [`ModrinthError::URLSerializationError`] if building the query string fails
    /// - [`ModrinthError::ReqwestError`] if the HTTP request or body stream fails
    /// - [`ModrinthError::IoError`] if writing to `writer` fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for other API errors
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};
    /// # use std::fs::File;
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let mut file = File::create("downloaded.jar")?;
    ///
    /// let written = client
    ///     .download_file_from_hash_to_writer(
    ///         "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///         VersionHashAlgorithm::Sha1,
    ///         None,
    ///         &mut file,
    ///     )
    ///     .await?;
    ///
    /// println!("downloaded {written} bytes");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file_from_hash_to_writer<S, W>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
        writer: &mut W,
    ) -> Result<u64, ModrinthError>
    where
        S: AsRef<str>,
        W: Write + ?Sized,
    {
        self.download_file_from_hash_to_writer_with_progress(
            hash,
            algorithm,
            version_id,
            writer,
            |_| {},
        )
        .await
    }

    /// Download a Modrinth version file by file hash into a standard writer with progress updates.
    ///
    /// This calls:
    /// `GET /version_file/{hash}/download`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/GET/v2/version_file/{version_id}/download>
    ///
    /// The `on_progress` callback is called once before downloading starts and again after each downloaded chunk is written.
    ///
    /// It is recommended to use asynchronous writes via [`ModrinthClient::download_file_from_hash_to_async_writer_with_progress`] over this function, however if you cannot use Tokio, don't care, or have some other reason this function is made available. Disabling the `tokio-io` feature will disable that function and its tokio usage.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// - [`ModrinthError::URLSerializationError`] if building the query string fails
    /// - [`ModrinthError::ReqwestError`] if the HTTP request or body stream fails
    /// - [`ModrinthError::IoError`] if writing to `writer` fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for other API errors
    pub async fn download_file_from_hash_to_writer_with_progress<S, W, F>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
        writer: &mut W,
        mut on_progress: F,
    ) -> Result<u64, ModrinthError>
    where
        S: AsRef<str>,
        W: Write + ?Sized,
        F: FnMut(DownloadProgress),
    {
        let mut resp = self
            .download_file_from_hash_response(hash, algorithm, version_id)
            .await?;

        let total = resp.content_length();
        let mut downloaded = 0u64;

        on_progress(DownloadProgress { downloaded, total });

        while let Some(chunk) = resp.chunk().await.map_err(ModrinthError::ReqwestError)? {
            writer.write_all(&chunk).map_err(ModrinthError::IoError)?;

            downloaded += chunk.len() as u64;

            on_progress(DownloadProgress { downloaded, total });
        }

        writer.flush().map_err(ModrinthError::IoError)?;

        Ok(downloaded)
    }
}
