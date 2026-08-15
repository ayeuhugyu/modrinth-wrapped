use serde::{Deserialize, Serialize};

/// Modrinth instance staging information
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ModrinthStagingInfo {
    pub about: String,
    pub documentation: String,
    pub name: String,
    pub version: String,
    pub build_info: BuildInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BuildInfo {
    pub comp_date: String,
    pub git_hash: String,
    pub profile: String,
}
