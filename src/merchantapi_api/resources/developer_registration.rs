//! Developer_registration resource
//!
//! Registers the GCP used for the API call to the shopping account passed in the request. Will create a user with an "API developer" and add the "developer_email" as a contact with "API notifications" email preference on.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Developer_registration resource handler
pub struct Developer_registration<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Developer_registration<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new developer_registration
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, developer_email: Option<String>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a developer_registration
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_developer_registration_operations() {
        // Test developer_registration CRUD operations
    }
}
