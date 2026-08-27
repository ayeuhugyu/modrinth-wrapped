#[cfg(feature = "loader-enums")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "loader-enums")]
/// A project loader.  
/// This list was generated using <https://api.modrinth.com/v2/tag/loader> on 2026-08-12.  
/// Note that modrinth may add more loaders in the future, which would make this list outdated.  
/// The [`Loader::Unknown`] variant works as a catch-all.  
///  
/// Requires the `loader-enums` feature.
pub enum Loader {
    Babric,
    BtaBabric,
    Bukkit,
    Bungeecord,
    Canvas,
    Datapack,
    Fabric,
    Folia,
    Forge,
    Geyser,
    Iris,
    JavaAgent,
    LegacyFabric,
    Liteloader,
    Minecraft,
    Modloader,
    Neoforge,
    Nilloader,
    Optifine,
    Ornithe,
    Paper,
    Purpur,
    Quilt,
    Rift,
    Spigot,
    Sponge,
    Vanilla,
    Velocity,
    Waterfall,

    Unknown(String),
}

#[cfg(feature = "loader-enums")]
impl Loader {
    /// Convert to string.
    /// For `Unknown(x)`, returns `x`.
    pub fn as_str(&self) -> &str {
        match self {
            Loader::Babric => "babric",
            Loader::BtaBabric => "bta-babric",
            Loader::Bukkit => "bukkit",
            Loader::Bungeecord => "bungeecord",
            Loader::Canvas => "canvas",
            Loader::Datapack => "datapack",
            Loader::Fabric => "fabric",
            Loader::Folia => "folia",
            Loader::Forge => "forge",
            Loader::Geyser => "geyser",
            Loader::Iris => "iris",
            Loader::JavaAgent => "java-agent",
            Loader::LegacyFabric => "legacy-fabric",
            Loader::Liteloader => "liteloader",
            Loader::Minecraft => "minecraft",
            Loader::Modloader => "modloader",
            Loader::Neoforge => "neoforge",
            Loader::Nilloader => "nilloaders",
            Loader::Optifine => "optifine",
            Loader::Ornithe => "ornithe",
            Loader::Paper => "paper",
            Loader::Purpur => "purpur",
            Loader::Quilt => "quilt",
            Loader::Rift => "rift",
            Loader::Spigot => "spigot",
            Loader::Sponge => "sponge",
            Loader::Vanilla => "vanilla",
            Loader::Velocity => "velocity",
            Loader::Waterfall => "waterfall",
            Loader::Unknown(other) => other.as_str(),
        }
    }

    /// Parse from `&str`
    pub fn from_str_ref(s: &str) -> Self {
        match s {
            "babric" => Loader::Babric,
            "bta-babric" => Loader::BtaBabric,
            "bukkit" => Loader::Bukkit,
            "bungeecord" => Loader::Bungeecord,
            "canvas" => Loader::Canvas,
            "datapack" => Loader::Datapack,
            "fabric" => Loader::Fabric,
            "folia" => Loader::Folia,
            "forge" => Loader::Forge,
            "geyser" => Loader::Geyser,
            "iris" => Loader::Iris,
            "java-agent" => Loader::JavaAgent,
            "legacy-fabric" => Loader::LegacyFabric,
            "liteloader" => Loader::Liteloader,
            "minecraft" => Loader::Minecraft,
            "modloader" => Loader::Modloader,
            "neoforge" => Loader::Neoforge,
            "nilloaders" => Loader::Nilloader,
            "optifine" => Loader::Optifine,
            "ornithe" => Loader::Ornithe,
            "paper" => Loader::Paper,
            "purpur" => Loader::Purpur,
            "quilt" => Loader::Quilt,
            "rift" => Loader::Rift,
            "spigot" => Loader::Spigot,
            "sponge" => Loader::Sponge,
            "vanilla" => Loader::Vanilla,
            "velocity" => Loader::Velocity,
            "waterfall" => Loader::Waterfall,
            other => Loader::Unknown(other.to_owned()),
        }
    }

    /// Parse from `String`
    fn from_string(s: String) -> Self {
        Self::from_str_ref(s.as_str())
    }
}

#[cfg(feature = "loader-enums")]
impl From<&str> for Loader {
    fn from(value: &str) -> Self {
        Loader::from_str_ref(value)
    }
}

#[cfg(feature = "loader-enums")]
impl From<String> for Loader {
    fn from(value: String) -> Self {
        Loader::from_string(value)
    }
}

#[cfg(feature = "loader-enums")]
impl<'de> Deserialize<'de> for Loader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Loader::from_string(s))
    }
}

#[cfg(feature = "loader-enums")]
impl Serialize for Loader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
