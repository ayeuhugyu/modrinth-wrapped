use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// A team member
pub struct TeamMember {
    /// The ID of the team this team member is a member of
    pub team_id: String,
    /// The base user
    pub user: User,
    /// The user's role on the team
    pub role: String,
    /// The user’s permissions in bitfield format
    ///
    /// This will always be [`None`], it is only available using Authorization headers which is not supported by this wrapper.
    pub permissions: Option<u32>,
    /// Whether or not the user has accepted to be on the team
    ///
    /// This will always be `true`, non-accepted members can only be viewed using Authorization headers which is not supported by this wrapper.
    pub accepted: bool,
    /// The split of payouts going to this user. The proportion of payouts they get is their split divided by the sum of the splits of all members.
    ///
    /// This will always be [`None`], it is only available using Authorization headers which is not supported by this wrapper.
    pub payouts_split: Option<u32>,
    /// The order of the team member.
    pub ordering: u32,
}
