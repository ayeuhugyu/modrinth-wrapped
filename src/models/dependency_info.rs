use serde::{Deserialize, Serialize};

use crate::{Project, Version};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DependencyInfo {
    /// Projects that the project depends upon
    pub projects: Vec<Project>,
    /// Versions that the project depends upon
    pub versions: Vec<Version>,
}
