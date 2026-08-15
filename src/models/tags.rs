use serde::{Deserialize, Serialize};

use crate::models::project::{DateField, ProjectType};

/// A category tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CategoryData {
    /// The SVG icon of a category
    pub icon: String,
    /// The name of the category
    pub name: String,
    /// The project type this category is applicable to
    pub project_type: ProjectType,
    /// The header under which the category should go
    pub header: String,
}

/// A loader tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoaderData {
    /// The SVG icon of a loader
    pub icon: String,
    /// The name of the loader
    pub name: String,
    /// The project types that this loader is applicable to
    pub supported_project_types: Vec<ProjectType>,
}

/// A game version tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GameVersionData {
    /// The name/number of the game version
    pub version: String,
    /// The name of the loader
    pub version_type: GameVersionType,
    /// The date of the game version release.  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    pub date: DateField,
    /// Whether or not this is a major version, used for Featured Versions
    pub major: bool,
}

/// The type of a game version tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Release,
    Snapshot,
    Alpha,
    Beta,
}

/// A license tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[deprecated(note = "simply use SPDX IDs.")]
pub struct LicenseData {
    /// The short identifier of the license
    pub short: String,
    /// The full name of the license
    pub name: String,
}

/// The text of a specific license
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LicenseTextData {
    /// The title of the license
    pub title: String,
    /// The body text of the license
    pub body: String,
}

/// A donation platform tag
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DonationPlatformData {
    /// The short identifier of the donation platform
    pub short: String,
    /// The full name of the donation platform
    pub name: String,
}
