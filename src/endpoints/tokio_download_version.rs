#[cfg(feature = "tokio-io")]
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::ModrinthClient;
#[cfg(feature = "tokio-io")]
use crate::{DownloadProgress, ModrinthError, VersionHashAlgorithm};

impl ModrinthClient {
    /// Download a version file by file hash into an async writer.
    ///
    /// This calls:
    /// `GET /version_file/{hash}/download`
    ///
    /// From <https://api.modrinth.com/docs#v2/tag/version-files/GET/v2/version_file/{version_id}/download>
    ///
    /// Unlike [`ModrinthClient::download_file_from_hash_to_writer`], this accepts [`tokio::io::AsyncWrite`], such as [`tokio::fs::File`].
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// - [`ModrinthError::URLSerializationError`] if building the query string fails (very unlikely)
    /// - [`ModrinthError::ReqwestError`] if the HTTP request or body stream fails
    /// - [`ModrinthError::IoError`] if writing to `writer` fails
    /// - [`ModrinthError::NotFound`] for 404 responses if the `not-found-error` feature is enabled
    /// - [`ModrinthError::ApiError`] for other API errors
    ///
    /// # Examples
    /// ```
    /// use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};
    ///
    /// async fn demo() -> Result<(), ModrinthError> {
    ///     let client = ModrinthClient::new()?;
    ///     let mut file = tokio::fs::File::create("downloaded.jar").await?;
    ///
    ///     client
    ///         .download_file_from_hash_to_async_writer(
    ///             "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///             VersionHashAlgorithm::Sha1,
    ///             None,
    ///             &mut file,
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "tokio-io")]
    pub async fn download_file_from_hash_to_async_writer<S, W>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
        writer: &mut W,
    ) -> Result<u64, ModrinthError>
    where
        S: AsRef<str>,
        W: AsyncWrite + Unpin + ?Sized,
    {
        self.download_file_from_hash_to_async_writer_with_progress(
            hash,
            algorithm,
            version_id,
            writer,
            |_| {},
        )
        .await
    }

    /// Download a version file by file hash into an async writer with progress updates.
    ///
    /// The `on_progress` callback is called once before downloading starts and again after each downloaded chunk is written.
    ///
    /// Returns the number of bytes written.
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
    /// #
    /// # async fn demo() -> Result<(), ModrinthError> {
    /// let client = ModrinthClient::new()?;
    /// let mut file = tokio::fs::File::create("downloaded.jar").await?;
    ///
    /// client
    ///     .download_file_from_hash_to_async_writer_with_progress(
    ///         "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///         VersionHashAlgorithm::Sha1,
    ///         None,
    ///         &mut file,
    ///         |progress| {
    ///             if let Some(total) = progress.total {
    ///                 println!("{} / {} bytes", progress.downloaded, total);
    ///             } else {
    ///                 println!("{} bytes", progress.downloaded);
    ///             }
    ///         },
    ///     )
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// With `indicatif`:
    /// ```
    /// use indicatif::ProgressBar;
    /// use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};
    ///
    /// async fn demo() -> Result<(), ModrinthError> {
    ///     let client = ModrinthClient::new()?;
    ///     let mut file = tokio::fs::File::create("downloaded.jar").await?;
    ///     let pb = ProgressBar::new(0);
    ///
    ///     client
    ///         .download_file_from_hash_to_async_writer_with_progress(
    ///             "3e0e0d37ea88fed51d684858a1bf24c287273e2d",
    ///             VersionHashAlgorithm::Sha1,
    ///             None,
    ///             &mut file,
    ///             |progress| {
    ///                 if let Some(total) = progress.total {
    ///                     pb.set_length(total);
    ///                 }
    ///
    ///                 pb.set_position(progress.downloaded);
    ///             },
    ///         )
    ///         .await?;
    ///
    ///     pb.finish();
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "tokio-io")]
    pub async fn download_file_from_hash_to_async_writer_with_progress<S, W, F>(
        &self,
        hash: S,
        algorithm: VersionHashAlgorithm,
        version_id: Option<&str>,
        writer: &mut W,
        mut on_progress: F,
    ) -> Result<u64, ModrinthError>
    where
        S: AsRef<str>,
        W: AsyncWrite + Unpin + ?Sized,
        F: FnMut(DownloadProgress),
    {
        let mut resp = self
            .download_file_from_hash_response(hash, algorithm, version_id)
            .await?;

        let total = resp.content_length();
        let mut downloaded = 0u64;

        on_progress(DownloadProgress { downloaded, total });

        while let Some(chunk) = resp.chunk().await.map_err(ModrinthError::ReqwestError)? {
            writer
                .write_all(&chunk)
                .await
                .map_err(ModrinthError::IoError)?;

            downloaded += chunk.len() as u64;

            on_progress(DownloadProgress { downloaded, total });
        }

        writer.flush().await.map_err(ModrinthError::IoError)?;

        Ok(downloaded)
    }
}
