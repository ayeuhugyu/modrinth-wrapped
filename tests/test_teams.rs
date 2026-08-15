use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn teams_get_project_team_members_fabric_api() -> Result<(), ModrinthError> {
    let c = client().await?;

    let members = c.get_project_team_members("fabric-api").await?;

    assert!(!members.is_empty());
    Ok(())
}

#[tokio::test]
async fn teams_get_project_team_members_sodium() -> Result<(), ModrinthError> {
    let c = client().await?;

    let members = c.get_project_team_members("sodium").await?;

    assert!(!members.is_empty());
    for m in &members {
        assert!(!m.team_id.is_empty());
        assert!(!m.role.is_empty());
        assert!(m.accepted || m.permissions.is_some());
    }

    Ok(())
}

#[tokio::test]
async fn teams_get_project_team_members_missing_slug_errors() -> Result<(), ModrinthError> {
    let c = client().await?;

    let missing = "this-should-not-exist-zzzzzzzzzzzzzzzzzz";
    let res = c.get_project_team_members(missing).await;

    match res {
        Ok(members) => {
            assert!(members.is_empty());
            Ok(())
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
