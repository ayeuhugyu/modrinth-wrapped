use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
/// The type of a version file
pub enum VersionFileType {
    RequiredResourcePack,
    OptionalResourcePack,
    SourcesJar,
    DevJar,
    JavadocJar,
    Signature,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// Hash types of a version file
pub enum VersionHashAlgorithm {
    Sha512,
    Sha1,
}

impl std::fmt::Display for VersionHashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionHashAlgorithm::Sha1 => write!(f, "sha1"),
            VersionHashAlgorithm::Sha512 => write!(f, "sha512"),
        }
    }
}

#[derive(Debug)]
pub struct ParseVersionHashTypeError(String);

impl std::fmt::Display for ParseVersionHashTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown VersionHashType: {}", self.0)
    }
}

impl std::error::Error for ParseVersionHashTypeError {}

impl FromStr for VersionHashAlgorithm {
    type Err = ParseVersionHashTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sha512" => Ok(VersionHashAlgorithm::Sha512),
            "sha1" => Ok(VersionHashAlgorithm::Sha1),
            other => Err(ParseVersionHashTypeError(other.to_string())),
        }
    }
}

impl TryFrom<String> for VersionHashAlgorithm {
    type Error = ParseVersionHashTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<VersionHashAlgorithm> for String {
    fn from(v: VersionHashAlgorithm) -> Self {
        match v {
            VersionHashAlgorithm::Sha512 => "sha512".to_string(),
            VersionHashAlgorithm::Sha1 => "sha1".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// Hashes of a version file
pub struct VersionFileHashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// A file for a version of a project.
pub struct VersionFile {
    /// A map of hashes of the file. The key is the hashing algorithm and the value is the string version of the hash.
    pub hashes: VersionFileHashes,
    /// A direct link to the file
    pub url: String,
    /// The name of the file
    pub filename: String,
    /// Whether this file is the primary one for its version.
    /// Only a maximum of one file per version will have this set to true.
    /// If there are not any primary files, it can be inferred that the first file is the primary one.
    pub primary: bool,
    /// The size of the file in bytes
    pub size: u32,
    /// The type of this version file
    pub file_type: Option<VersionFileType>,
}
