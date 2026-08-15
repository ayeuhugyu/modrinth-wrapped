use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Forge update information
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ForgeUpdates {
    /// The project's homepage
    pub homepage: String,
    /// The promoted updates of the project
    ///
    /// Formatted in a list of minecraft versions latest/recommended mapped to the respective version number
    ///
    /// # Examples:
    /// - `promos.get("1.19.2-latest") == String::from("8.3.134+forge")`
    /// - `promos.get("1.19.2-recommended") == String::from("8.3.134+forge")`
    ///
    /// Note: this is only for forge versions, and is primarily meant to be used by the [Forge Update Checker](https://docs.minecraftforge.net/en/latest/misc/updatechecker/)
    pub promos: HashMap<String, String>,
}
