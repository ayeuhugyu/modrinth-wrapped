use modrinth_wrapped::{VersionHashQuery, VersionType};

#[cfg(feature = "loader-enums")]
use modrinth_wrapped::Loader;

#[cfg(feature = "loader-enums")]
pub type LoaderField = Loader;

#[cfg(not(feature = "loader-enums"))]
pub type LoaderField = String;

fn make_loaders_fabric() -> Vec<LoaderField> {
    #[cfg(feature = "loader-enums")]
    {
        vec![Loader::Fabric]
    }
    #[cfg(not(feature = "loader-enums"))]
    {
        vec!["fabric".to_string()]
    }
}

#[test]
fn version_hash_query_default_filters_are_none() {
    let q = VersionHashQuery::new();

    let (gv, loaders, vt) = q.filters_as_optional_refs();

    assert!(gv.is_none());
    assert!(loaders.is_none());
    assert!(vt.is_none());
}

#[test]
fn version_hash_query_with_game_versions_sets_only_that_filter() {
    let q = VersionHashQuery::new().with_game_versions(vec!["1.20.1".to_string()]);

    let (gv, loaders, vt) = q.filters_as_optional_refs();

    assert!(gv.is_some());
    assert!(loaders.is_none());
    assert!(vt.is_none());

    assert_eq!(gv.unwrap(), &vec!["1.20.1".to_string()]);
}

#[test]
fn version_hash_query_with_loaders_sets_loaders_filter() {
    let q = VersionHashQuery::new().with_loaders(make_loaders_fabric());

    let (gv, loaders, vt) = q.filters_as_optional_refs();

    assert!(gv.is_none());
    assert!(loaders.is_some());
    assert!(vt.is_none());
}

#[test]
fn version_hash_query_with_version_types_sets_version_types_filter() {
    let q = VersionHashQuery::new().with_version_types(vec![VersionType::Release]);

    let (gv, loaders, vt) = q.filters_as_optional_refs();

    assert!(gv.is_none());
    assert!(loaders.is_none());
    assert!(vt.is_some());
    assert_eq!(vt.unwrap(), &vec![VersionType::Release]);
}

#[test]
fn version_hash_query_with_hash_sets_hash_only() {
    let q = VersionHashQuery::new().with_hash("SOME_HASH");

    assert_eq!(q.hash.as_deref(), Some("SOME_HASH"));
    let (gv, loaders, vt) = q.filters_as_optional_refs();
    assert!(gv.is_none());
    assert!(loaders.is_none());
    assert!(vt.is_none());
}

#[test]
fn version_hash_query_with_all_filters() {
    let q = VersionHashQuery::new()
        .with_hash("SOME_HASH")
        .with_game_versions(vec!["1.20.1".to_string(), "1.19.4".to_string()])
        .with_loaders(make_loaders_fabric())
        .with_version_types(vec![VersionType::Release, VersionType::Beta]);

    let (gv, loaders, vt) = q.filters_as_optional_refs();

    assert!(gv.is_some());
    assert!(loaders.is_some());
    assert!(vt.is_some());
}
