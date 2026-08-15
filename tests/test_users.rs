use modrinth_wrapped::{ModrinthClient, ModrinthError};

async fn client() -> Result<ModrinthClient, ModrinthError> {
    ModrinthClient::new()
}

#[tokio::test]
async fn users_get_user_by_username_modmuss50() -> Result<(), ModrinthError> {
    let c = client().await?;

    let user = c.get_user("modmuss50").await?;

    assert!(!user.id.is_empty());
    assert!(!user.username.is_empty());
    Ok(())
}

#[tokio::test]
async fn users_get_user_by_username_jellysquid3() -> Result<(), ModrinthError> {
    let c = client().await?;

    let user = c.get_user("jellysquid3").await?;

    assert!(!user.id.is_empty());
    assert!(!user.username.is_empty());
    Ok(())
}

#[tokio::test]
async fn users_get_user_missing_username_errors() -> Result<(), ModrinthError> {
    let c = client().await?;

    let missing = "this-should-not-exist-zzzzzzzzzzzzzzzzzz";
    let res = c.get_user(missing).await;

    match res {
        Ok(user) => {
            assert!(user.id.is_empty());
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

#[tokio::test]
async fn users_get_users_bulk_two_usernames() -> Result<(), ModrinthError> {
    let c = client().await?;

    let users = c.get_users(vec!["modmuss50", "jellysquid3"]).await?;

    assert!(!users.is_empty());
    Ok(())
}

#[tokio::test]
async fn users_get_users_bulk_with_duplicates() -> Result<(), ModrinthError> {
    let c = client().await?;

    let users = c
        .get_users(vec!["modmuss50", "modmuss50", "modmuss50"])
        .await?;

    assert!(!users.is_empty());
    Ok(())
}

#[tokio::test]
async fn users_get_user_projects_modmuss50() -> Result<(), ModrinthError> {
    let c = client().await?;

    let projects = c.get_user_projects("modmuss50").await?;

    assert!(!projects.is_empty());
    Ok(())
}

#[tokio::test]
async fn users_get_user_projects_jellysquid3() -> Result<(), ModrinthError> {
    let c = client().await?;

    let projects = c.get_user_projects("jellysquid3").await?;

    assert!(!projects.is_empty());
    Ok(())
}
