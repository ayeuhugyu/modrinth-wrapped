use reqwest::{Error, Response, StatusCode};
use serde::Deserialize;

#[derive(Debug)]
/// A modrinth error.
///
/// The [`ModrinthError::NotFound`] variant can be disabled by
/// turning off the `not-found-error` feature.
/// This will make those errors use [`ModrinthError::ApiError`] instead.
pub enum ModrinthError {
    InvalidUserAgent,
    IoError(std::io::Error),
    ClientCreateError(reqwest::Error),
    InvalidHeaderValue, // this should never happen
    ReqwestError(reqwest::Error),
    JSONSerializationError(serde_json::Error),
    URLSerializationError(serde_urlencoded::ser::Error),
    #[cfg(feature = "not-found-error")]
    NotFound,
    ApiError {
        error: String,
        description: String,
        status: StatusCode,
    },
    MissingHash,
}

impl From<std::io::Error> for ModrinthError {
    fn from(value: std::io::Error) -> Self {
        return Self::IoError(value);
    }
}

impl From<reqwest::Error> for ModrinthError {
    fn from(value: reqwest::Error) -> Self {
        return Self::ReqwestError(value);
    }
}

impl From<serde_json::Error> for ModrinthError {
    fn from(value: serde_json::Error) -> Self {
        return Self::JSONSerializationError(value);
    }
}

impl From<serde_urlencoded::ser::Error> for ModrinthError {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        return Self::URLSerializationError(value);
    }
}

#[derive(Debug, Deserialize)]
pub struct ModrinthApiError {
    pub error: String,
    pub description: String,
}

impl std::fmt::Display for ModrinthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModrinthError::InvalidUserAgent => write!(f, "Invalid User-Agent"),
            ModrinthError::ClientCreateError(e) => {
                write!(f, "Reqwest error creating an HTTP client: {e}")
            }
            ModrinthError::IoError(e) => write!(f, "IO Error: {e}"),
            ModrinthError::ReqwestError(e) => write!(f, "Reqwest error: {e}"),
            ModrinthError::JSONSerializationError(e) => write!(f, "JSON Serialization error: {e}"),
            ModrinthError::URLSerializationError(e) => write!(f, "URL Serialization error: {e}"),
            #[cfg(feature = "not-found-error")]
            ModrinthError::NotFound => write!(f, "404 not found"),
            ModrinthError::InvalidHeaderValue => write!(f, "Invalid header value"),
            ModrinthError::ApiError {
                error,
                description,
                status,
            } => write!(
                f,
                "API error: {{ status: {status}, error: {error}, description: {description} }}"
            ),
            ModrinthError::MissingHash => write!(
                f,
                "A VersionHashQuery was not provided a Hash for latest_versions_with_individual_filters. Use `VersionHashQuery.with_hash` to set one."
            ),
        }
    }
}

pub(crate) async fn map_response_error(
    resp_raw: Result<Response, Error>,
) -> Result<Response, ModrinthError> {
    let resp = match resp_raw {
        Ok(r) => r,
        Err(e) => return Err(ModrinthError::ReqwestError(e)),
    };

    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }

    let raw = resp.text().await.ok();

    let parsed: Option<ModrinthApiError> = raw
        .as_deref()
        .and_then(|s| serde_json::from_str::<ModrinthApiError>(s).ok());

    #[cfg(feature = "not-found-error")]
    {
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ModrinthError::NotFound);
        }
    }

    match parsed {
        Some(parsed) => {
            return Err(ModrinthError::ApiError {
                status,
                error: parsed.error,
                description: parsed.description,
            });
        }
        None => {
            return Err(ModrinthError::ApiError {
                status: StatusCode::from_u16(400).unwrap(),
                error: "couldnt_parse_error".into(),
                description: "modrinth-wrapped couldn't parse modrinth's api error".into(),
            });
        }
    }
}

impl std::error::Error for ModrinthError {}
