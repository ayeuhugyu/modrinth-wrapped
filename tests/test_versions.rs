use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionQuery};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

async fn get_some_versions(
    c: &ModrinthClient,
) -> Result<Vec<modrinth_wrapped::Version>, ModrinthError> {
    let versions = c.list_versions_for("fabric-api", None).await?;

    Ok(versions)
}

#[tokio::test]
async fn versions_list_versions_for_none_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;
    let versions = c.list_versions_for("fabric-api", None).await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn versions_list_versions_for_with_game_versions_filter_smoke() -> Result<(), ModrinthError> {
    let c = client().await?;

    let q = VersionQuery::new().with_game_versions(vec!["1.20.1".to_string()]);

    let versions = c.list_versions_for("fabric-api", Some(&q)).await?;

    assert!(!versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn versions_get_version_roundtrip_from_list_versions() -> Result<(), ModrinthError> {
    let c = client().await?;

    let versions = get_some_versions(&c).await?;
    let v0 = versions
        .first()
        .cloned()
        .ok_or_else(|| panic!("no versions returned"))
        .map_err(|_| ModrinthError::NotFound)?;

    let fetched = c.get_version(v0.id.clone()).await?;

    assert_eq!(fetched.id, v0.id);
    Ok(())
}

#[tokio::test]
async fn versions_get_versions_bulk_two_ids() -> Result<(), ModrinthError> {
    let c = client().await?;

    let versions = get_some_versions(&c).await?;
    assert!(versions.len() >= 2);

    let v0 = versions[0].id.clone();
    let v1 = versions[1].id.clone();

    let bulk = c.get_versions(vec![v0.clone(), v1.clone()]).await?;

    let returned_ids: Vec<String> = bulk.iter().map(|v| v.id.clone()).collect();

    assert!(returned_ids.contains(&v0));
    assert!(returned_ids.contains(&v1));

    Ok(())
}

#[tokio::test]
async fn versions_get_project_version_via_number_accepts_id() -> Result<(), ModrinthError> {
    let c = client().await?;

    let versions = get_some_versions(&c).await?;
    let v0 = versions
        .first()
        .cloned()
        .ok_or_else(|| panic!("no versions returned"))
        .map_err(|_| ModrinthError::NotFound)?;

    let fetched = c
        .get_project_version_via_number("fabric-api", v0.id.clone())
        .await?;

    assert_eq!(fetched.id, v0.id);
    Ok(())
}

#[tokio::test]
async fn versions_get_project_version_via_number_accepts_version_number()
-> Result<(), ModrinthError> {
    let c = client().await?;

    let versions = get_some_versions(&c).await?;
    let v0 = versions
        .first()
        .cloned()
        .ok_or_else(|| panic!("no versions returned"))
        .map_err(|_| ModrinthError::NotFound)?;

    let version_number = v0.version_number.clone();

    let fetched = c
        .get_project_version_via_number("fabric-api", version_number)
        .await;

    match fetched {
        Ok(v) => {
            assert!(!v.id.is_empty());
            Ok(())
        }
        Err(e) => Err(e),
    }
}
