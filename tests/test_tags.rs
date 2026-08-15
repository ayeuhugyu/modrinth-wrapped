use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn tags_list_tag_categories_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let categories = c.list_tag_categories().await?;
    assert!(!categories.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_loaders_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let loaders = c.list_tag_loaders().await?;
    assert!(!loaders.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_game_versions_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let versions = c.list_tag_game_versions().await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
#[allow(deprecated)]
async fn tags_list_tag_licenses_smoke_and_fetch_license_text() -> Result<(), ModrinthError> {
    let c = client().await?;

    #[allow(deprecated)]
    let licenses = c.list_tag_licenses().await?;

    assert!(!licenses.is_empty());

    let first = &licenses[0];
    let license_id = first.short.clone();

    let text = c.get_license_text(license_id.clone()).await?;

    assert!(!text.title.is_empty());
    assert!(!text.body.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_donation_platforms_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let platforms = c.list_tag_donation_platforms().await?;

    assert!(!platforms.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_report_types_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let report_types = c.list_tag_report_types().await?;

    assert!(!report_types.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_project_types_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let project_types = c.list_tag_project_types().await?;

    assert!(!project_types.is_empty());
    Ok(())
}

#[tokio::test]
async fn tags_list_tag_side_types_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;

    #[allow(deprecated)]
    let side_types = c.list_tag_side_types().await?;

    assert!(!side_types.is_empty());
    Ok(())
}
