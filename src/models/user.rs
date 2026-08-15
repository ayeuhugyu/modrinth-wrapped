use serde::{Deserialize, Serialize};

use crate::models::project::DateField;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// A Modrinth user.
pub struct User {
    /// The user's username
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub email: Option<String>,
    /// A description of the user
    pub bio: Option<String>,
    /// Various data relating to the user’s payouts status
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub payout_data: Option<UserPayoutData>,
    /// The user's ID
    pub id: String,
    /// The user's avatar url
    pub avatar_url: String,
    /// The time at which the user was created.  
    /// Disabling the `parse-dates` feature will leave this as a ISO-8601 string.
    #[serde(rename = "created")]
    pub date_created: DateField,
    /// The user's role
    pub role: UserRole,
    /// Any badges applicable to this user. These are currently unused and undisplayed, and as such are subject to change
    ///
    /// In order from first to seventh bit, the current bits are:
    ///
    /// - (unused)
    /// - EARLY_MODPACK_ADOPTER
    /// - EARLY_RESPACK_ADOPTER
    /// - EARLY_PLUGIN_ADOPTER
    /// - ALPHA_TESTER
    /// - CONTRIBUTOR
    /// - TRANSLATOR
    ///
    /// Format: `bitfield`
    pub badges: u32,
    /// A list of authentication providers you have signed up for
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub auth_providers: Option<Vec<String>>,
    /// Whether your email is verified
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub email_verified: Option<bool>,
    /// Whether you have a password associated with your account
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub has_password: Option<bool>,
    /// Whether you have TOTP two-factor authentication connected to your account
    ///
    /// This will always be [`None`], it is only available if displaying your own account using Authorization headers which is not supported by this wrapper.
    pub has_totp: Option<bool>,
    #[deprecated(note = "this is no longer public for security reasons and is always None.")]
    pub github_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// The role of a user
pub enum UserRole {
    Admin,
    Moderator,
    Developer,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// # user payout data.
///
/// This struct is left empty as it will never be shown in public apis.  
/// It is pointless to add the structure for it since it will never be used in this wrapper.
pub struct UserPayoutData {}
