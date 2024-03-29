use std::collections::HashMap;

use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};

/// Result status of authentication flow
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AuthStatus {
    /// Authentication flow completed successfully, resulting in attributes
    #[serde(rename = "success")]
    Success,
    /// Authentication flow completed unsuccessfully, no attributes were
    /// obtained
    #[serde(rename = "failed")]
    Failed,
}

/// Result of an authentication flow
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AuthResult {
    /// Status of the result
    pub status: AuthStatus,
    /// Attribute jwe containing the obtained attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
    /// URL on which the authentication plugin wants to be kept updated on
    /// session status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_url: Option<String>,
}

/// Session activity status update type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, FromFormField)]
pub enum SessionActivity {
    /// User has had sufficient activity to extend session timeout
    #[serde(rename = "user_active")]
    #[field(value = "user_active")]
    UserActive,
    /// User has indicated desire to logout
    #[serde(rename = "logout")]
    #[field(value = "logout")]
    Logout,
}
