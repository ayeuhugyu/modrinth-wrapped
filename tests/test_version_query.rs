#[cfg(feature = "loader-enums")]
use modrinth_wrapped::Loader;
use modrinth_wrapped::{ModrinthError, VersionQuery};

#[cfg(not(feature = "loader-enums"))]
pub type LoaderField = String;

#[cfg(feature = "loader-enums")]
pub type LoaderField = Loader;

fn make_query() -> VersionQuery {
    VersionQuery::new()
}

#[test]
fn version_query_default_omits_loaders_and_game_versions() -> Result<(), ModrinthError> {
    let q = make_query();

    let url = q.to_url_query()?;

    assert!(!url.contains("featured="));

    assert!(!url.contains("loaders="));
    assert!(!url.contains("game_versions="));

    assert!(url.contains("include_changelog=false"));

    Ok(())
}

#[test]
fn version_query_with_game_versions_includes_game_versions_param() -> Result<(), ModrinthError> {
    let q =
        VersionQuery::new().with_game_versions(vec!["1.20.1".to_string(), "1.19.4".to_string()]);

    let url = q.to_url_query()?;

    assert!(url.contains("game_versions="));
    assert!(url.contains("include_changelog=false"));
    assert!(!url.contains("loaders="));

    Ok(())
}

#[test]
fn version_query_with_empty_loaders_still_omits_loaders_param() -> Result<(), ModrinthError> {
    let empty_loaders: Vec<LoaderField> = Vec::new();
    let q = VersionQuery::new().with_loaders(empty_loaders);

    let url = q.to_url_query()?;

    assert!(!url.contains("loaders="));
    assert!(!url.contains("game_versions="));
    assert!(url.contains("include_changelog=false"));

    Ok(())
}

#[test]
fn version_query_with_loaders_includes_loaders_param() -> Result<(), ModrinthError> {
    let q = {
        #[cfg(feature = "loader-enums")]
        {
            VersionQuery::new().with_loaders(vec![Loader::Fabric])
        }

        #[cfg(not(feature = "loader-enums"))]
        {
            VersionQuery::new().with_loaders(vec!["fabric".to_string()])
        }
    };

    let url = q.to_url_query()?;

    assert!(url.contains("loaders="));
    assert!(!url.contains("game_versions="));
    assert!(url.contains("include_changelog=false"));

    Ok(())
}

#[test]
fn version_query_featured_and_changelog_flags_are_encoded() -> Result<(), ModrinthError> {
    let q = VersionQuery::new()
        .with_featured(Some(true))
        .with_changelog(true);

    let url = q.to_url_query()?;

    assert!(url.contains("featured=true"));
    assert!(url.contains("include_changelog=true"));

    Ok(())
}
