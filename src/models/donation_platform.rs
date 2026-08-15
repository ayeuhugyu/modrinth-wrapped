#[cfg(feature = "donation-platform-enums")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "donation-platform-enums")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DonationPlatform {
    Patreon,
    BuyMeACoffee,
    PayPal,
    GitHubSponsors,
    KoFi,
    Other,

    Unknown(String),
}

#[cfg(feature = "donation-platform-enums")]
impl DonationPlatform {
    fn as_str(&self) -> &str {
        match self {
            DonationPlatform::Patreon => "patreon",
            DonationPlatform::BuyMeACoffee => "bmac",
            DonationPlatform::PayPal => "paypal",
            DonationPlatform::GitHubSponsors => "github",
            DonationPlatform::KoFi => "ko-fi",
            DonationPlatform::Other => "other",
            DonationPlatform::Unknown(other) => other.as_str(),
        }
    }

    fn from_str_ref(s: &str) -> Self {
        match s {
            "patreon" => DonationPlatform::Patreon,
            "bmac" => DonationPlatform::BuyMeACoffee,
            "paypal" => DonationPlatform::PayPal,
            "github" => DonationPlatform::GitHubSponsors,
            "ko-fi" => DonationPlatform::KoFi,
            "other" => DonationPlatform::Other,
            other => DonationPlatform::Unknown(other.to_owned()),
        }
    }

    fn from_string(s: String) -> Self {
        match s.as_str() {
            "patreon" => DonationPlatform::Patreon,
            "bmac" => DonationPlatform::BuyMeACoffee,
            "paypal" => DonationPlatform::PayPal,
            "github" => DonationPlatform::GitHubSponsors,
            "ko-fi" => DonationPlatform::KoFi,
            "other" => DonationPlatform::Other,
            _ => DonationPlatform::Unknown(s),
        }
    }
}

#[cfg(feature = "donation-platform-enums")]
impl From<&str> for DonationPlatform {
    fn from(value: &str) -> Self {
        DonationPlatform::from_str_ref(value)
    }
}

#[cfg(feature = "donation-platform-enums")]
impl From<String> for DonationPlatform {
    fn from(value: String) -> Self {
        DonationPlatform::from_string(value)
    }
}

#[cfg(feature = "donation-platform-enums")]
impl<'de> Deserialize<'de> for DonationPlatform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(DonationPlatform::from_string(s))
    }
}

#[cfg(feature = "donation-platform-enums")]
impl Serialize for DonationPlatform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
