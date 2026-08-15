use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// An environment in which a version is available in.  
/// See <https://modrinth.com/news/article/new-environments/> for further details.
pub enum Environment {
    /// **Client-side only** (`client_only`)
    ///
    /// All functionality is performed exclusively on the client side.
    /// Should be compatible with vanilla servers.
    ClientOnly,
    /// **Server-side only / Works in singleplayer** (`server_only`)
    ///
    /// All functionality is performed exclusively on the server side.
    /// Should be compatible with vanilla clients if only installed on the server.
    /// Also works in Singleplayer.
    ServerOnly,
    /// **Server-side only / Dedicated server onl**y (`dedicated_server_only`)
    ///
    /// Only runs on a dedicated server, and not in Singleplayer.
    DedicatedServerOnly,
    /// **Client and server / Required on both** (`client_and_server`)
    ///
    /// Must be installed on both the client and server.
    ClientAndServer,
    /// **Client and server / Optional on client**
    /// (`server_only_client_optional`)
    ///
    /// Must be on the server, but can be on the client as well for enhanced functionality
    ServerOnlyClientOptional,
    /// **Client and server / Optional on server**
    /// (`client_only_server_optional`)
    ///
    /// Must be on the client, but can be on the server as well for enhanced functionality
    ClientOnlyServerOptional,
    /// **Client or server / Works best on both**
    /// (`client_or_server_prefers_both`)
    ///
    /// Can be installed on just the client or just the server to function, but functionality is enhanced when it is on both.
    ClientOrServerPrefersBoth,
    /// **Client or server / Works the same on either** (`client_or_server`)
    ///
    /// Can be installed on just the client or just the server, and either one would enable full functionality.
    /// There would be no reason to install it on both.
    ClientOrServer,
    /// **Singleplayer only** (`singleplayer_only`)
    ///
    /// Only works in Singleplayer, does not function in a Multiplayer environment.
    SingleplayerOnly,
    /// Unknown / Unspecified
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[deprecated(note = "use [`Environment`](crate::Environment) instead")]
pub enum OldEnvironment {
    Required,
    Optional,
    Unsupported,
    Unknown,
}
