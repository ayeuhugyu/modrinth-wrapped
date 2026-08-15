use serde::{Deserialize, Serialize};

use crate::models::project::{DateField, LoaderField};
use crate::{Environment, VersionFile};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The type of a dependency
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The release channel of a version
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// A specific version of a project that a version depends on.
/// Not officially documented, but at least one of `version_id` or `project_id` can be expected to be defined.
pub struct Dependency {
    /// The ID of the version that this version depends on
    pub version_id: Option<String>,
    /// The ID of the project that this version depends on
    pub project_id: Option<String>,
    /// The file name of the dependency, mostly used for showing external dependencies on modpacks
    pub file_name: Option<String>,
    /// The type of dependency that this version has
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The status of a project
pub enum VersionStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// A version of a project.
pub struct Version {
    /// The name of this version
    pub name: String,
    /// The version number. Ideally will follow semantic versioning
    pub version_number: String,
    /// The changelog for this version
    pub changelog: Option<String>,
    /// A list of specific versions of projects that this version depends on
    pub dependencies: Vec<Dependency>,
    /// A list of versions of Minecraft that this version supports
    pub game_versions: Vec<String>,
    /// The release channel for this version
    pub version_type: VersionType,

    /// A list of all of the loaders supported by the project. These vary based
    /// on project type.
    pub loaders: Vec<LoaderField>,

    /// Whether the version is featured or not
    pub featured: bool,
    /// Status of this version
    pub status: VersionStatus,
    /// Requested status of this version
    pub requested_status: Option<VersionStatus>,
    /// The ID of the version, encoded as a base62 string
    pub id: String,
    /// The ID of the project this version is for
    pub project_id: String,
    /// The ID of the author who published this version
    pub author_id: String,
    /// The date this version was published at.  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "date_published")]
    pub date_created: DateField,
    /// The number of times this version has been downloaded
    pub downloads: u32,
    /// A link to the changelog for this version. Always null, only kept for legacy compatibility.
    #[deprecated]
    pub changelog_url: Option<String>,
    /// The environment a project or version supports.
    /// For an explanation of each environment, see the blog post here: <https://modrinth.com/news/article/new-environments/#new-system>
    pub environment: Environment,
    /// A list of files available for download for this version
    pub files: Vec<VersionFile>,
}
