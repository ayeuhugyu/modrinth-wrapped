use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn staging_modrinth_staging_info_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;

    let info = c.modrinth_staging_info().await?;

    assert!(!info.about.is_empty());
    assert!(!info.name.is_empty());
    assert!(!info.version.is_empty());

    // BuildInfo sanity checks
    assert!(!info.build_info.git_hash.is_empty());
    assert!(!info.build_info.profile.is_empty());

    Ok(())
}
