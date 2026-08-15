use modrinth_wrapped::{
    ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionHashQuery, VersionType,
};

#[cfg(feature = "loader-enums")]
use modrinth_wrapped::Loader;

#[cfg(feature = "loader-enums")]
pub type LoaderField = Loader;

#[cfg(not(feature = "loader-enums"))]
pub type LoaderField = String;

fn fabric_loaders() -> Vec<LoaderField> {
    #[cfg(feature = "loader-enums")]
    {
        vec![Loader::Fabric]
    }
    #[cfg(not(feature = "loader-enums"))]
    {
        vec!["fabric".to_string()]
    }
}

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn version_files_get_version_from_hash_sha1() -> Result<(), ModrinthError> {
    let c = client().await?;

    let hash = "3e0e0d37ea88fed51d684858a1bf24c287273e2d";
    let v = c
        .get_version_from_hash(hash, VersionHashAlgorithm::Sha1)
        .await?;

    assert!(!v.id.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_get_projects_from_hashes_sha1() -> Result<(), ModrinthError> {
    let c = client().await?;

    let hashes = vec![
        "3e0e0d37ea88fed51d684858a1bf24c287273e2d".to_string(),
        "fe1389e3e68ffebdc94d5f7db41d87b81654467f".to_string(),
    ];

    let projects = c
        .get_projects_from_hashes(hashes, VersionHashAlgorithm::Sha1)
        .await?;

    assert!(!projects.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_get_versions_from_hashes_sha1() -> Result<(), ModrinthError> {
    let c = client().await?;

    let hashes = vec![
        "3e0e0d37ea88fed51d684858a1bf24c287273e2d".to_string(),
        "fe1389e3e68ffebdc94d5f7db41d87b81654467f".to_string(),
    ];

    let versions = c
        .get_versions_from_hashes(hashes, VersionHashAlgorithm::Sha1)
        .await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_latest_compatible_version_from_hash_no_filters() -> Result<(), ModrinthError>
{
    let c = client().await?;

    let hash = "3e0e0d37ea88fed51d684858a1bf24c287273e2d";

    let v = c
        .latest_compatible_version_from_hash(hash, VersionHashAlgorithm::Sha1, None, None)
        .await?;

    assert!(!v.id.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_latest_compatible_version_from_hash_with_filters()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let hash = "3e0e0d37ea88fed51d684858a1bf24c287273e2d";

    let q = VersionHashQuery::new()
        .with_game_versions(vec!["1.20.1".to_string()])
        .with_loaders(fabric_loaders())
        .with_version_types(vec![VersionType::Release]);

    let v = c
        .latest_compatible_version_from_hash(hash, VersionHashAlgorithm::Sha1, Some(&q), None)
        .await?;

    assert!(!v.id.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_latest_compatible_version_from_hashes_with_filters()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let hashes = vec![
        "2824689367eec2b52c6085e968343751137e66bf".to_string(),
        "89c79de1c8e17f3d34fded3a2fe401e7e60a707d".to_string(),
    ];

    let q = VersionHashQuery::new().with_game_versions(vec!["1.20.1".to_string()]);

    let versions = c
        .latest_compatible_version_from_hashes(hashes, VersionHashAlgorithm::Sha1, Some(&q))
        .await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_all_latest_compatible_versions_from_hashes_with_filters()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let hashes = vec![
        "586ac174c831da9fd35edce1e36eb14c7f878ce3",
        "84f19148833a29fadac9e0405596bcca502d3a53",
    ];

    let q = VersionHashQuery::new()
        .with_game_versions(vec!["1.19.4"])
        .with_loaders(fabric_loaders())
        .with_version_types(vec![VersionType::Release]);

    let versions = c
        .all_latest_compatible_versions_from_hashes(hashes, VersionHashAlgorithm::Sha1, Some(&q))
        .await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_latest_versions_with_individual_filters_happy_path()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let q1 = VersionHashQuery::new()
        .with_hash("2824689367eec2b52c6085e968343751137e66bf")
        .with_game_versions(vec!["1.20.1".to_string()])
        .with_loaders(fabric_loaders())
        .with_version_types(vec![VersionType::Release]);

    let q2 = VersionHashQuery::new()
        .with_hash("89c79de1c8e17f3d34fded3a2fe401e7e60a707d")
        .with_game_versions(vec!["1.19.4".to_string()])
        .with_loaders(fabric_loaders())
        .with_version_types(vec![VersionType::Release]);

    let queries = vec![q1.clone(), q2.clone()];

    let versions = c
        .latest_versions_with_individual_filters(VersionHashAlgorithm::Sha1, queries)
        .await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn version_files_latest_versions_with_individual_filters_missing_hash_errors()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let q = VersionHashQuery::new()
        .with_game_versions(vec!["1.20.1".to_string()])
        .with_loaders(fabric_loaders())
        .with_version_types(vec![VersionType::Release]);

    let res = c
        .latest_versions_with_individual_filters(VersionHashAlgorithm::Sha1, vec![q])
        .await;

    match res {
        Err(ModrinthError::MissingHash) => Ok(()),
        other => panic!("Expected MissingHash error, got: {other:?}"),
    }
}
