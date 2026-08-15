use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm};

const HASH: &str = "3e0e0d37ea88fed51d684858a1bf24c287273e2d";

fn temp_file_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "modrinth_wrapped_{label}_{}_{}.jar",
        std::process::id(),
        nanos,
    ))
}

#[tokio::test]
async fn download_file_from_hash_response_returns_raw_stream() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;

    let mut response = client
        .download_file_from_hash_response(&hash, VersionHashAlgorithm::Sha1, version_id.as_deref())
        .await?;

    assert!(
        response.status().is_success(),
        "expected successful response, got {}",
        response.status()
    );

    let mut downloaded = 0usize;

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(ModrinthError::ReqwestError)?
    {
        downloaded += chunk.len();
    }

    assert!(downloaded > 0, "downloaded response stream was empty");

    Ok(())
}

#[tokio::test]
async fn download_file_from_hash_bytes_downloads_into_memory() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;

    let bytes = client
        .download_file_from_hash_bytes(&hash, VersionHashAlgorithm::Sha1, version_id.as_deref())
        .await?;

    assert!(!bytes.is_empty(), "downloaded bytes were empty");

    Ok(())
}

#[tokio::test]
async fn download_file_from_hash_to_writer_downloads_into_vec() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;

    let mut output = Vec::new();

    let written = client
        .download_file_from_hash_to_writer(
            &hash,
            VersionHashAlgorithm::Sha1,
            version_id.as_deref(),
            &mut output,
        )
        .await?;

    assert!(written > 0, "reported zero bytes written");
    assert_eq!(
        written as usize,
        output.len(),
        "reported byte count did not match writer length"
    );

    Ok(())
}

#[tokio::test]
async fn download_file_from_hash_to_writer_downloads_into_std_file() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;
    let path = temp_file_path("std_writer");

    let result = async {
        let mut file = std::fs::File::create(&path).map_err(ModrinthError::IoError)?;

        let written = client
            .download_file_from_hash_to_writer(
                &hash,
                VersionHashAlgorithm::Sha1,
                version_id.as_deref(),
                &mut file,
            )
            .await?;

        drop(file);

        let metadata = std::fs::metadata(&path).map_err(ModrinthError::IoError)?;

        assert!(written > 0, "reported zero bytes written");
        assert_eq!(
            written,
            metadata.len(),
            "reported byte count did not match file length"
        );

        Ok(())
    }
    .await;

    let _ = std::fs::remove_file(&path);

    result
}

#[tokio::test]
async fn download_file_from_hash_to_writer_with_progress_reports_progress()
-> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;

    let mut output = Vec::new();

    let mut callback_count = 0usize;
    let mut last_downloaded = 0u64;
    let mut final_downloaded = 0u64;
    let mut final_total = None;

    let written = client
        .download_file_from_hash_to_writer_with_progress(
            &hash,
            VersionHashAlgorithm::Sha1,
            version_id.as_deref(),
            &mut output,
            |progress| {
                callback_count += 1;

                assert!(
                    progress.downloaded >= last_downloaded,
                    "progress went backwards: previous={}, current={}",
                    last_downloaded,
                    progress.downloaded
                );

                if let Some(total) = progress.total {
                    assert!(
                        progress.downloaded <= total,
                        "downloaded bytes exceeded reported total"
                    );
                }

                if let Some(fraction) = progress.fraction() {
                    assert!(
                        (0.0..=1.0).contains(&fraction),
                        "progress fraction was outside 0.0..=1.0: {fraction}"
                    );
                }

                last_downloaded = progress.downloaded;
                final_downloaded = progress.downloaded;
                final_total = progress.total;
            },
        )
        .await?;

    assert!(written > 0, "reported zero bytes written");
    assert_eq!(
        written as usize,
        output.len(),
        "reported byte count did not match writer length"
    );
    assert_eq!(
        written, final_downloaded,
        "final progress did not match reported written byte count"
    );

    // One initial progress call at 0 bytes, then at least one chunk.
    assert!(
        callback_count >= 2,
        "expected at least initial and final progress callbacks"
    );

    if let Some(total) = final_total {
        assert_eq!(
            total, written,
            "reported content length did not match final written byte count"
        );
    }

    Ok(())
}

#[cfg(feature = "tokio-io")]
#[tokio::test]
async fn download_file_from_hash_to_async_writer_downloads_into_tokio_file()
-> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;
    let path = temp_file_path("tokio_async_writer");

    let result = async {
        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(ModrinthError::IoError)?;

        let written = client
            .download_file_from_hash_to_async_writer(
                &hash,
                VersionHashAlgorithm::Sha1,
                version_id.as_deref(),
                &mut file,
            )
            .await?;

        drop(file);

        let metadata = tokio::fs::metadata(&path)
            .await
            .map_err(ModrinthError::IoError)?;

        assert!(written > 0, "reported zero bytes written");
        assert_eq!(
            written,
            metadata.len(),
            "reported byte count did not match file length"
        );

        Ok(())
    }
    .await;

    let _ = tokio::fs::remove_file(&path).await;

    result
}

#[cfg(feature = "tokio-io")]
#[tokio::test]
async fn download_file_from_hash_to_async_writer_with_progress_reports_progress()
-> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;
    let hash = HASH;
    let version_id: Option<String> = None;
    let path = temp_file_path("tokio_async_writer_progress");

    let result = async {
        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(ModrinthError::IoError)?;

        let mut callback_count = 0usize;
        let mut last_downloaded = 0u64;
        let mut final_downloaded = 0u64;
        let mut final_total = None;

        let written = client
            .download_file_from_hash_to_async_writer_with_progress(
                &hash,
                VersionHashAlgorithm::Sha1,
                version_id.as_deref(),
                &mut file,
                |progress| {
                    callback_count += 1;

                    assert!(
                        progress.downloaded >= last_downloaded,
                        "progress went backwards: previous={}, current={}",
                        last_downloaded,
                        progress.downloaded
                    );

                    if let Some(total) = progress.total {
                        assert!(
                            progress.downloaded <= total,
                            "downloaded bytes exceeded reported total"
                        );
                    }

                    if let Some(fraction) = progress.fraction() {
                        assert!(
                            (0.0..=1.0).contains(&fraction),
                            "progress fraction was outside 0.0..=1.0: {fraction}"
                        );
                    }

                    last_downloaded = progress.downloaded;
                    final_downloaded = progress.downloaded;
                    final_total = progress.total;
                },
            )
            .await?;

        drop(file);

        let metadata = tokio::fs::metadata(&path)
            .await
            .map_err(ModrinthError::IoError)?;

        assert!(written > 0, "reported zero bytes written");
        assert_eq!(
            written,
            metadata.len(),
            "reported byte count did not match file length"
        );
        assert_eq!(
            written, final_downloaded,
            "final progress did not match reported written byte count"
        );

        assert!(
            callback_count >= 2,
            "expected at least initial and final progress callbacks"
        );

        if let Some(total) = final_total {
            assert_eq!(
                total, written,
                "reported content length did not match final written byte count"
            );
        }

        Ok(())
    }
    .await;

    let _ = tokio::fs::remove_file(&path).await;

    result
}
