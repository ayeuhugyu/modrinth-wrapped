use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn make_client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn projects_get_project_by_slug_fabric_api() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let project = client.get_project("fabric-api").await?;

    assert!(!project.id.is_empty());
    assert!(project.slug.as_deref() == Some("fabric-api") || project.slug.is_none());
    assert!(!project.title.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_project_by_slug_sodium() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let project = client.get_project("sodium").await?;

    assert!(!project.id.is_empty());
    assert!(project.title.to_lowercase().contains("sodium") || !project.title.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_project_by_id_roundtrip_from_slug() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let project = client.get_project("fabric-api").await?;

    let project_by_id = client.get_project(project.id.clone()).await?;

    assert_eq!(project.id, project_by_id.id);
    Ok(())
}

#[tokio::test]
async fn projects_get_projects_bulk_two_slugs() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client.get_projects(vec!["fabric-api", "sodium"]).await?;

    assert!(!projects.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_projects_bulk_three_slugs() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client
        .get_projects(vec!["fabric-api", "sodium", "cloth-config"])
        .await?;

    assert!(!projects.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_projects_bulk_with_duplicates() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client
        .get_projects(vec!["fabric-api", "fabric-api", "fabric-api"])
        .await?;

    assert!(!projects.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_random_projects_count_5() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client.get_random_projects(5).await?;

    assert_eq!(projects.len(), 5);
    Ok(())
}

#[tokio::test]
async fn projects_get_random_projects_count_10() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client.get_random_projects(10).await?;

    assert_eq!(projects.len(), 10);
    Ok(())
}

#[tokio::test]
async fn projects_verify_project_existing_slug_fabric_api() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let verify = client.verify_project("fabric-api").await?;

    assert!(!verify.id.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_verify_project_existing_id_roundtrip() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let project = client.get_project("fabric-api").await?;

    let verify = client.verify_project(project.id.clone()).await?;

    assert!(!verify.id.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_verify_project_missing_slug() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let missing = "this-should-not-exist-zzzzzzzzzzzzzzzzzzzz";

    let res = client.verify_project(missing).await;
    match res {
        Ok(_) => {
            panic!("verify_project(\"{}\") returned OK", missing);
        }
        #[cfg(feature = "not-found-error")]
        Err(ModrinthError::NotFound) => Ok(()),
        Err(ModrinthError::ApiError {
            error: _,
            description: _,
            status: _,
        }) => Ok(()),
        Err(other) => Err(other),
    }
}

#[tokio::test]
async fn projects_get_dependencies_for_sodium_slug() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let deps = client.get_dependencies_for("sodium").await?;

    assert!(!deps.projects.is_empty() || !deps.versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_dependencies_for_sodium_id() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let project = client.get_project("sodium").await?;

    let deps = client.get_dependencies_for(project.id.clone()).await?;

    assert!(!deps.projects.is_empty() || !deps.versions.is_empty());
    Ok(())
}

#[tokio::test]
async fn projects_get_projects_then_verify_each_returned_id() -> Result<(), ModrinthError> {
    let client = make_client().await?;

    let projects = client.get_projects(vec!["fabric-api", "sodium"]).await?;

    for p in &projects {
        let verify = client.verify_project(p.id.clone()).await?;

        assert!(!verify.id.is_empty());
    }

    Ok(())
}
