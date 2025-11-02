//! User_deletion_request resource
//!
//! Insert or update a user deletion requests.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// User_deletion_request resource handler
pub struct User_deletion_request<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> User_deletion_request<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new user_deletion_request
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, property_id: Option<String>, id: Option<String>, deletion_request_time: Option<String>, firebase_project_id: Option<String>, kind: Option<String>, web_property_id: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_deletion_request_operations() {
        // Test user_deletion_request CRUD operations
    }
}
