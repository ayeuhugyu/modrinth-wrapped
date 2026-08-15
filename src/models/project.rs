#[cfg(feature = "parse-dates")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "donation-platform-enums")]
use crate::models::donation_platform::DonationPlatform;
#[cfg(feature = "loader-enums")]
use crate::models::loader::Loader;
#[allow(deprecated)]
use crate::models::{
    environment::{Environment, OldEnvironment},
    gallery_item::GalleryItem,
};

#[cfg(feature = "parse-dates")]
pub type DateField = DateTime<Utc>;

#[cfg(not(feature = "parse-dates"))]
pub type DateField = String;

#[cfg(feature = "loader-enums")]
pub type LoaderField = Loader;

#[cfg(not(feature = "loader-enums"))]
pub type LoaderField = String;

#[cfg(feature = "donation-platform-enums")]
pub type DonationPlatformField = DonationPlatform;

#[cfg(not(feature = "donation-platform-enums"))]
pub type DonationPlatformField = String;

/// The license of a project
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProjectLicense {
    /// The SPDX license ID of a project
    id: String,
    /// The long name of a license
    name: String,
    /// The URL to this license
    url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The project type of a project
pub enum ProjectType {
    /// # used only by modloader tags.
    Project,
    Mod,
    ModPack,
    ResourcePack,
    DataPack,
    Shader,
    Plugin,
    #[serde(rename = "minecraft_java_server")]
    MinecraftJavaServer,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The status of a project
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Withheld,
    Scheduled,
    Private,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
/// The monetization status of a project
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}

/// A message that a moderator sent regarding a project
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[deprecated]
pub struct ModeratorMessage {
    /// The message that a moderator has left for the project
    message: String,
    /// The longer body of the message that a moderator has left for the project
    body: Option<String>,
}

/// A donation link for a project
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DonationUrl {
    /// The ID of the donation platform
    id: DonationPlatformField,
    /// The donation platform this link is to
    platform: String,
    /// The URL of the donation platform and user
    url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[allow(deprecated)]
/// A project.
pub struct Project {
    /// The ID of the project, encoded as a base62 string
    pub id: String,
    /// The ID of the team that has ownership of this project
    pub team: String,
    /// The title or name of the project
    pub title: String,
    /// A short sentence summarizing the project, no more than a sentence or two.
    pub description: String,
    /// A long form description of the project
    pub body: String,
    /// The status of the project
    ///
    /// A status decides the visibility of a project in search, URLs, and the whole site itself.
    /// * Approved - Project is displayed on search, and accessible by URL
    /// * Rejected - Project is not displayed on search, and not accessible by URL (Temporary state, project can reapply)
    /// * Draft - Project is not displayed on search, and not accessible by URL
    /// * Unlisted - Project is not displayed on search, but accessible by URL
    /// * Withheld - Same as unlisted, but set by a moderator. Cannot be switched to another type without moderator approval
    /// * Processing - Project is not displayed on search, and not accessible by URL (Temporary state, project under review)
    /// * Scheduled - Project is scheduled to be released in the future
    /// * Private - Project is approved, but is not viewable to the public
    pub status: ProjectStatus,
    /// The project type of the project
    pub project_type: ProjectType,
    /// A list of the featured categories that the project has
    pub categories: Vec<String>,
    /// A list of additional categories that the project also has.
    /// These are supplementary to the featured categories, and does not include them again.
    pub additional_categories: Vec<String>,
    /// All the environments that versions of this project support.
    /// Not in any particular order, we recommend using the environment information on a version instead.
    /// For an explanation of each environment, see the blog post here: <https://modrinth.com/news/article/new-environments/#new-system>
    pub environment: Vec<Environment>,
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,

    /// A list of all of the loaders supported by the project. These vary based
    /// on project type.
    pub loaders: Vec<LoaderField>,

    /// A list of the version IDs of the project
    pub versions: Vec<String>,
    /// The license of the project
    pub license: ProjectLicense,
    /// The date the project was created  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "published")]
    pub date_created: DateField,
    /// The date the latest version of the project was created  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "updated")]
    pub date_modified: DateField,
    /// The total number of downloads of the project
    pub downloads: u64,
    /// The total number of users following the project
    pub followers: u64,
    /// A list of images that have been uploaded to the project’s gallery
    pub gallery: Vec<GalleryItem>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: String,
    /// The monetization status of the project
    pub monetization_status: MonetizationStatus,
    /// The slug of a project, used for vanity URLs.
    /// Regex: `^[\w!@$()`.+,"\-']{3,64}$`
    pub slug: Option<String>,
    /// The ID of the organization that owns this project
    pub organization: Option<String>,
    /// The requested status when submitting for review or scheduling the
    /// project for release.
    ///
    /// A status decides the visibility of a project in search, URLs, and the whole site itself.
    /// * Approved - Project is displayed on search, and accessible by URL
    /// * Rejected - Project is not displayed on search, and not accessible by URL (Temporary state, project can reapply)
    /// * Draft - Project is not displayed on search, and not accessible by URL
    /// * Unlisted - Project is not displayed on search, but accessible by URL
    /// * Withheld - Same as unlisted, but set by a moderator. Cannot be switched to another type without moderator approval
    /// * Processing - Project is not displayed on search, and not accessible by URL (Temporary state, project under review)
    /// * Scheduled - Project is scheduled to be released in the future
    /// * Private - Project is approved, but is not viewable to the public
    pub requested_status: Option<ProjectStatus>,
    /// The date the project was first published
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "approved")]
    pub date_approved: Option<DateField>,
    /// The date the project’s status was submitted to moderators for review  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "queued")]
    pub date_queued: Option<DateField>,
    /// The URL of the project's icon
    pub icon_url: Option<String>,
    /// The URL of the project’s icon without CDN transforms applied
    pub raw_icon_url: Option<String>,

    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<u32>,

    /// An optional link to where to submit bugs or issues with the project
    pub issues_url: Option<String>,
    /// An optional link to the source code of the project
    pub source_url: Option<String>,
    /// An optional link to the project’s wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the project’s discord.
    pub discord_url: Option<String>,
    /// A list of donation links for the project
    pub donation_urls: Option<Vec<DonationUrl>>,

    #[deprecated(note = "use `Project.environment` instead")]
    pub client_side: OldEnvironment,
    #[deprecated(note = "use `Project.environment` instead")]
    pub server_side: OldEnvironment,

    /// The link to the long description of the project. Always null, only kept for legacy compatibility.
    #[deprecated]
    pub body_url: Option<String>,

    /// A message that a moderator sent regarding the project
    #[deprecated]
    pub moderator_message: Option<ModeratorMessage>,
}
