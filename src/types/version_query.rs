use serde::Serialize;

use crate::ModrinthError;
use crate::models::project::LoaderField;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// A version filter query to be used with the `Client.list_versions_for()` function.
///
/// Any empty filter vectors are **omitted** from the JSON body entirely.
pub struct VersionQuery {
    /// The types of loaders to filter for.
    pub loaders: Vec<LoaderField>,
    /// The game versions to filter for.
    pub game_versions: Vec<String>,
    /// Allows to filter for featured or non-featured versions only. Will be omitted if set to None.
    pub featured: Option<bool>,
    /// Allows you to toggle the inclusion of the changelog field in the response.
    /// It is highly recommended to use include_changelog=false in most cases unless you specifically need the changelog for all versions.
    pub include_changelog: bool,
}

impl VersionQuery {
    pub fn new() -> Self {
        return Self {
            loaders: vec![],
            game_versions: vec![],
            featured: None,
            include_changelog: false,
        };
    }

    pub fn with_loaders<L: Into<LoaderField>>(mut self, loaders: Vec<L>) -> Self {
        self.loaders = loaders.into_iter().map(Into::into).collect();
        return self;
    }
    pub fn with_game_versions<S: Into<String>>(mut self, game_versions: Vec<S>) -> Self {
        self.game_versions = game_versions.into_iter().map(Into::into).collect();
        return self;
    }
    pub fn with_featured(mut self, featured: Option<bool>) -> Self {
        self.featured = featured;
        return self;
    }
    pub fn with_changelog(mut self, include_changelog: bool) -> Self {
        self.include_changelog = include_changelog;
        return self;
    }

    pub fn to_url_query(&self) -> Result<String, ModrinthError> {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            loaders: Option<String>, // JSON string
            #[serde(skip_serializing_if = "Option::is_none")]
            game_versions: Option<String>, // JSON string
            #[serde(skip_serializing_if = "Option::is_none")]
            featured: Option<bool>,
            include_changelog: bool,
        }

        let loaders_final = if self.loaders.is_empty() {
            None
        } else {
            Some(
                serde_json::to_string(&self.loaders)
                    .map_err(ModrinthError::JSONSerializationError)?,
            )
        };

        let game_versions_final = if self.game_versions.is_empty() {
            None
        } else {
            Some(
                serde_json::to_string(&self.game_versions)
                    .map_err(ModrinthError::JSONSerializationError)?,
            )
        };

        let params = Params {
            loaders: loaders_final,
            game_versions: game_versions_final,
            featured: self.featured,
            include_changelog: self.include_changelog,
        };

        return serde_urlencoded::to_string(&params).map_err(ModrinthError::URLSerializationError);
        // return Ok(encoded) as Result<String, ModrinthError>;
    }
}
