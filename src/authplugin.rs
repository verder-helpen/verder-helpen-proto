use serde::{Deserialize, Serialize};

/// Data sent along in a start_authentication request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartAuthRequest {
    /// Attributes to request from user
    pub attributes: Vec<String>,
    /// URL to which to redirect user after completion of authentication flow
    pub continuation: String,
    /// URL to which authentication result will/should be sent after completion
    /// of authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_url: Option<String>,
}

/// Result expected from a start_authentication request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartAuthResponse {
    /// URL for user to start authentication flow
    pub client_url: String,
}
