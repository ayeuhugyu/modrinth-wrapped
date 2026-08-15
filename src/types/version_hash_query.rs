use crate::models::project::LoaderField;
use crate::models::version::VersionType;

/// Filters used by "compatible version" endpoints.
///
/// - Any empty filter vectors are omitted from the JSON body entirely.
/// - `hash` is optional and is intended for the endpoint that needs an
///   explicit per-hash "latest versions with individual filters".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VersionHashQuery {
    /// Filter to versions compatible with these game versions.
    pub game_versions: Vec<String>,
    /// Filter to specific loaders.
    pub loaders: Vec<LoaderField>,
    /// Filter to specific version types.
    pub version_types: Vec<VersionType>,
    /// Optional hash used by the "latest versions with individual filters" API.
    pub hash: Option<String>,
}

pub type FiltersAsOptionalRefs<'a> = (
    Option<&'a Vec<String>>,
    Option<&'a Vec<LoaderField>>,
    Option<&'a Vec<VersionType>>,
);

impl VersionHashQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_game_versions<S: Into<String>>(mut self, game_versions: Vec<S>) -> Self {
        self.game_versions = game_versions.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_loaders<L: Into<LoaderField>>(mut self, loaders: Vec<L>) -> Self {
        self.loaders = loaders.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_version_types(mut self, version_types: Vec<VersionType>) -> Self {
        self.version_types = version_types;
        self
    }

    /// used only for the individual-filter endpoint.
    pub fn with_hash<S: Into<String>>(mut self, hash: S) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn filters_as_optional_refs(&self) -> FiltersAsOptionalRefs<'_> {
        let game_versions = (!self.game_versions.is_empty()).then_some(&self.game_versions);
        let loaders = (!self.loaders.is_empty()).then_some(&self.loaders);
        let version_types = (!self.version_types.is_empty()).then_some(&self.version_types);
        (game_versions, loaders, version_types)
    }
}
