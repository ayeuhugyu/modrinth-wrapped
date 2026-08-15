use serde::{Deserialize, Serialize};

use crate::Environment;
#[allow(deprecated)]
use crate::models::environment::OldEnvironment;
use crate::models::project::{DateField, ProjectType};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[allow(deprecated)]
/// A search hit.
pub struct SearchHit {
    /// The ID of the project, encoded as a base62 string
    pub project_id: String,
    /// The project type of the project
    pub project_type: ProjectType,
    /// All project types across every version of the project, unlike project_type which only reflects a version-specific type
    pub all_project_types: Vec<ProjectType>,
    /// The title or name of the project
    pub title: String,
    /// A short sentence summarizing the project, no more than a sentence or two.
    pub description: String,
    /// The username of the project's author
    pub author: String,
    /// A list of the featured categories that the project has
    pub categories: Vec<String>,
    /// A list of the featured categories that the project has. Equivalent to `categories` on the project itself.
    pub display_categories: Vec<String>,
    /// A list of the minecraft versions supported by the project
    #[serde(rename = "versions")]
    pub game_versions: Vec<String>,
    /// The total number of downloads of the project
    pub downloads: u64,
    /// The total number of users following the project
    #[serde(rename = "follows")]
    pub followers: u64,
    /// The URL of the project's icon
    pub icon_url: Option<String>,
    /// The date the project was created  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    pub date_created: DateField,
    /// The date the latest version of the project was created  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    pub date_modified: DateField,
    /// The ID of the latest version of the project
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: String,
    /// All the environments that versions of this project support.
    /// Not in any particular order, we recommend using the environment information on a version instead.
    /// For an explanation of each environment, see the blog post here: <https://modrinth.com/news/article/new-environments/#new-system>
    pub environment: Vec<Environment>,
    /// A list of URLs for images that have been uploaded to the project’s gallery
    pub gallery: Vec<String>,
    /// The slug of a project, used for vanity URLs.
    /// Regex: `^[\w!@$()`.+,"\-']{3,64}$`
    pub slug: Option<String>,
    /// The ID of the project’s author
    pub author_id: Option<String>,
    /// The name of the organization that owns this project
    #[serde(rename = "organization")]
    pub organization_name: Option<String>,
    /// The ID of the organization that owns this project
    #[serde(rename = "organization_id")]
    pub organization: Option<String>,
    /// The URL of the featured gallery image of the project
    pub featured_gallery: Option<String>,

    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<u32>,

    #[deprecated(note = "use `Project.environment` instead")]
    pub client_side: OldEnvironment,
    #[deprecated(note = "use `Project.environment` instead")]
    pub server_side: OldEnvironment,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// A search result.
pub struct SearchResult {
    /// Search hits.
    pub hits: Vec<SearchHit>,
    /// How many results were skipped.
    pub offset: u32,
    /// The limit used for this query
    pub limit: u8,
    /// The total number of hits, ignoring the value of `limit`.
    pub total_hits: u32,
}
