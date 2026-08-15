use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// The response for the verify project endpoint.
pub struct VerifyResponse {
    /// The ID of the project
    pub id: String,
}
