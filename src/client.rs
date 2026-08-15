#[cfg(feature = "rate-limited")]
use std::sync::Arc;

use reqwest::header;
#[cfg(feature = "rate-limited")]
use tokio::{
    sync::Semaphore,
    time::{Duration, sleep},
};

use crate::error::ModrinthError;

const DEFAULT_URL: &str = "https://api.modrinth.com/v2";
const DEFAULT_STAGING_URL: &str = "https://staging-api.modrinth.com/";
const DEFAULT_FORGE_UPDATES_URL: &str = "https://api.modrinth.com/";

#[cfg(feature = "rate-limited")]
const DEFAULT_RATE_LIMIT: Duration = Duration::from_millis(200);

#[cfg(feature = "rate-limited")]
pub(crate) struct RateLimiter {
    sem: Arc<Semaphore>,
    delay: Duration,
}

#[cfg(feature = "rate-limited")]
impl RateLimiter {
    pub(crate) fn new(rate: Duration) -> Self {
        Self {
            sem: Arc::new(Semaphore::new(1)),
            delay: rate,
        }
    }

    pub(crate) async fn acquire(&self) -> tokio::sync::OwnedSemaphorePermit {
        let permit = self.sem.clone().acquire_owned().await.unwrap();
        sleep(self.delay).await;
        permit
    }

    pub(crate) fn set_rate_limit(&mut self, rate: Duration) -> &mut Self {
        self.delay = rate;
        return self;
    }
}

/// A Modrinth API client.
pub struct ModrinthClient {
    pub(crate) http_client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) staging_url: String,
    pub(crate) forge_updates_base_url: String,
    #[cfg(feature = "rate-limited")]
    pub(crate) rate_limiter: RateLimiter,
}

impl ModrinthClient {
    /// Creates a new modrinth client.
    ///
    /// # Errors
    /// - [`ModrinthError::ClientCreateError`] if the HTTP client cannot be built.
    /// - [`ModrinthError::InvalidHeaderValue`] if the automated `Content-Type: application/json` header is unable to be added. (very unlikely)
    pub fn new() -> Result<Self, ModrinthError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/json")
                .map_err(|_| ModrinthError::InvalidHeaderValue)?,
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ModrinthError::ClientCreateError)?;

        return Ok(ModrinthClient {
            http_client,
            base_url: DEFAULT_URL.into(),
            staging_url: DEFAULT_STAGING_URL.into(),
            forge_updates_base_url: DEFAULT_FORGE_UPDATES_URL.into(),
            #[cfg(feature = "rate-limited")]
            rate_limiter: RateLimiter::new(DEFAULT_RATE_LIMIT),
        });
    }

    /// Creates a new modrinth client and sets the User-Agent to the specified user agent.
    ///
    /// Read <https://docs.modrinth.com/api/#user-agents> for further info.
    /// # Errors
    /// - [`ModrinthError::InvalidUserAgent`] if `user_agent` cannot be converted into a valid `User-Agent` header value.
    /// - [`ModrinthError::InvalidHeaderValue`] if the automated `Content-Type: application/json` header is unable to be added. (very unlikely)
    /// - [`ModrinthError::ClientCreateError`] if the HTTP client cannot be built.
    pub fn with_user_agent(user_agent: &str) -> Result<Self, ModrinthError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(user_agent)
                .map_err(|_| ModrinthError::InvalidUserAgent)?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str("application/json")
                .map_err(|_| ModrinthError::InvalidHeaderValue)?,
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ModrinthError::ClientCreateError)?;

        return Ok(ModrinthClient {
            http_client,
            base_url: DEFAULT_URL.into(),
            staging_url: DEFAULT_STAGING_URL.into(),
            forge_updates_base_url: DEFAULT_FORGE_UPDATES_URL.into(),
            #[cfg(feature = "rate-limited")]
            rate_limiter: RateLimiter::new(DEFAULT_RATE_LIMIT),
        });
    }

    /// Creates a new modrinth client using the provided http client.
    ///
    /// NOTE: when using this function, you will need to set the `Content-Type` header to `application/json` yourself.  
    /// A few endpoints return errors if you do not correctly set `Content-Type`.
    pub fn with_http_client(http_client: reqwest::Client) -> Self {
        return ModrinthClient {
            http_client,
            base_url: DEFAULT_URL.into(),
            staging_url: DEFAULT_STAGING_URL.into(),
            forge_updates_base_url: DEFAULT_FORGE_UPDATES_URL.into(),
            #[cfg(feature = "rate-limited")]
            rate_limiter: RateLimiter::new(DEFAULT_RATE_LIMIT),
        };
    }

    /// Sets the base url used for all API requests.
    ///
    /// The default value is `https://api.modrinth.com/v2`.
    pub fn set_base_url<S: Into<String>>(mut self, new_url: S) -> Self {
        self.base_url = new_url.into();
        return self;
    }

    /// Sets the base url used for the forge_updates endpoint.
    ///
    /// The default value is `https://api.modrinth.com/`.
    pub fn set_forge_updates_url<S: Into<String>>(mut self, new_url: S) -> Self {
        self.forge_updates_base_url = new_url.into();
        return self;
    }

    /// Sets the base url used for the modrinth_staging_info endpoint.
    ///
    /// The default value is `https://staging-api.modrinth.com/`.
    pub fn set_staging_info_url<S: Into<String>>(mut self, new_url: S) -> Self {
        self.staging_url = new_url.into();
        return self;
    }

    #[cfg(feature = "rate-limited")]
    /// Sets the base rate limit used for all requests.
    ///
    /// The default value is 200ms AKA 300 requests per minute, as per modrinth documentation.
    pub fn set_rate_limit(mut self, rate: Duration) -> Self {
        self.rate_limiter.set_rate_limit(rate);
        return self;
    }
}
