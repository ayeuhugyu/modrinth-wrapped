use serde::{Deserialize, Serialize};

/// Various Modrinth statistics.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ModrinthStatistics {
    /// Number of projects on Modrinth
    pub projects: u32,
    /// Number of versions on Modrinth
    pub versions: u32,
    /// Number of version files on Modrinth
    pub files: u32,
    /// Number of authors (users with projects) on Modrinth
    pub authors: u32,
}
