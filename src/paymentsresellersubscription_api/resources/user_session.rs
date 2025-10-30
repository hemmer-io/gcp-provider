//! User_session resource
//!
//! This API replaces user authorized OAuth consent based APIs (Create, Entitle). Issues a timed session token for the given user intent. You can use the session token to redirect the user to Google to finish the signup flow. You can re-generate new session token repeatedly for the same request if necessary, regardless of the previous tokens being expired or not. By default, the session token is valid for 1 hour.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// User_session resource handler
pub struct User_session<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> User_session<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new user_session
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, intent_payload: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_session_operations() {
        // Test user_session CRUD operations
    }
}
