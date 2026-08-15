use modrinth_wrapped::{ForgeUpdatesQueryType, ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

const SLUG: &str = "cloth-config";

#[tokio::test]
async fn forge_updates_get_forge_only() -> Result<(), ModrinthError> {
    let c = client().await?;

    let updates = c
        .get_forge_updates(SLUG, ForgeUpdatesQueryType::ForgeOnly)
        .await?;

    assert!(!updates.homepage.is_empty());
    assert!(!updates.promos.is_empty());

    Ok(())
}

#[tokio::test]
async fn forge_updates_get_neoforge_only() -> Result<(), ModrinthError> {
    let c = client().await?;

    let updates = c
        .get_forge_updates(SLUG, ForgeUpdatesQueryType::NeoForgeOnly)
        .await?;

    assert!(!updates.homepage.is_empty());
    assert!(!updates.promos.is_empty());

    Ok(())
}

#[tokio::test]
async fn forge_updates_get_both() -> Result<(), ModrinthError> {
    let c = client().await?;

    let updates = c
        .get_forge_updates(SLUG, ForgeUpdatesQueryType::Both)
        .await?;

    assert!(!updates.homepage.is_empty());
    assert!(!updates.promos.is_empty());

    Ok(())
}

#[tokio::test]
async fn forge_updates_get_missing_slug_errors() -> Result<(), ModrinthError> {
    let c = client().await?;

    let missing = "this-should-not-exist-zzzzzzzzzzzzzzzzzz";
    let res = c
        .get_forge_updates(missing, ForgeUpdatesQueryType::ForgeOnly)
        .await;

    match res {
        Ok(_) => {
            panic!("expected error for missing slug");
        }
        Err(ModrinthError::ApiError {
            error: _,
            description: _,
            status: _,
        }) => Ok(()),
        Err(ModrinthError::ReqwestError(_)) => Ok(()),
        Err(e) => Err(e),
    }
}
