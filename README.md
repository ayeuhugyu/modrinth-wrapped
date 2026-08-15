# modrinth-wrapped
[![Crates.io downloads][crates-downloads-badge]][crates-url]
[![Crates.io version][crates-version-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][ci-badge]][ci-url]

[crates-downloads-badge]: https://img.shields.io/crates/d/modrinth-wrapped
[crates-version-badge]: https://img.shields.io/crates/v/modrinth-wrapped
[crates-url]: https://crates.io/crates/modrinth-wrapped
[mit-badge]: https://img.shields.io/badge/license-MIT-blue
[mit-url]: https://github.com/ayeuhugyu/modrinth-wrapped/blob/master/LICENSE
[ci-badge]: https://img.shields.io/badge/github-repo-orange?logo=github
[ci-url]: https://github.com/ayeuhugyu/modrinth-wrapped/actions?query=workflow%3ACI+branch%3Amaster

An async rust wrapper for the public Modrinth API (v2).  
Does not provide any functionality for endpoints requiring Authorization.

## Features
- `rate-limited`: (default) Enables per-client rate limiting.  
Note: uses [Tokio](https://tokio.rs) to rate limit requests. If you cannot or don't want to use Tokio, you can disable this feature and implement a custom rate limiter. See the [Modrinth API Docs](https://docs.modrinth.com/api/#ratelimits) for more information.
- `tokio-io`: (default) Enables async functions for downloading version files.  
If you cannot or don't want to use Tokio, you can disable this feature and implement a custom downloader using `ModrinthClient.download_file_from_hash_response`.
- `parse-dates`: (default) Enables parsing ISO-8601 date fields into `chrono::DateTime`.
- `not-found-error`: (default) Enables treating HTTP 404 responses as a dedicated `ModrinthError::NotFound` variant
- `loader-enums`: Enables the `Loader` enum, which will be used to deserialize/serialize loader fields instead of raw strings.
- `donation-platform-enums`: Enables the `DonationPlatform` enum, which will be used to deserialize donation platform fields instead of raw strings.

# Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
modrinth-wrapped = "0.1"
tokio = { version = "1.53.1", features = ["full"] } # at minimum you will need "macros" and "rt-multi-thread"
```

# Examples
These examples use Tokio, but you can theoretically use any async runtime you'd like.

### Fetch a single project by its ID or slug:
```rust
use modrinth_wrapped::{ModrinthClient, ModrinthError};

#[tokio::main]
async fn main() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;

    let project = client.get_project("fabric-api").await?;
    println!("project: {:#?}", project.title);

    Ok(())
}
```

### Search for projects:
```rust
use modrinth_wrapped::{ModrinthClient, SearchQuery, ModrinthError};

#[tokio::main]
async fn main() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;

    let query = SearchQuery::new()
        .with_query("fabric")
        .with_limit(5); // default limit is 10, max is 100

    let results = client.search(&query).await?;
    for hit in results.hits {
        println!("hit: {}", hit.title);
    }

    Ok(())
}
```

### Download a version file:
For this example, the latest `1.20.1` version for `fabric-api` is being downloaded.  
For displaying progress as it is downloaded, see `download_file_hash_to_async_writer_with_progress`.
```rust
use modrinth_wrapped::{ModrinthClient, ModrinthError, VersionHashAlgorithm, VersionQuery};

#[tokio::main]
async fn main() -> Result<(), ModrinthError> {
    let client = ModrinthClient::new()?;

    let query = VersionQuery::new().with_game_versions(vec!["1.20.1"]);
    let versions = client.list_versions_for("fabric-api", Some(&query)).await?;
    let latest_file = versions
        .first()
        .and_then(|version| version.files.first())
        .unwrap();

    let mut file = tokio::fs::File::create(&latest_file.filename)
        .await
        .map_err(ModrinthError::IoError)?;

    let bytes_written = client
        .download_file_from_hash_to_async_writer(
            &latest_file.hashes.sha1,
            VersionHashAlgorithm::Sha1,
            None,
            &mut file,
        )
        .await?;

    println!("downloaded {bytes_written} bytes");

    Ok(())
}
```

## License
Licensed under the MIT license.  
See [LICENSE](LICENSE)