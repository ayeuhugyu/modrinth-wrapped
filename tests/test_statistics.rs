use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn statistics_instance_statistics_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;

    let stats = c.instance_statistics().await?;

    assert!(stats.projects > 0);
    assert!(stats.versions > 0);
    assert!(stats.files > 0);
    assert!(stats.authors > 0);

    Ok(())
}
