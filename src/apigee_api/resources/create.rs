//! Create resource
//!
//! Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API. **Note**: All keys start out with status=approved, even if status=revoked is passed when the key is created. To revoke a key, use the UpdateDeveloperAppKey API.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Create resource handler
pub struct Create<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Create<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new create
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, attributes: Option<Vec<String>>, expires_in_seconds: Option<String>, consumer_key: Option<String>, expires_at: Option<String>, scopes: Option<Vec<String>>, status: Option<String>, issued_at: Option<String>, api_products: Option<Vec<String>>, consumer_secret: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_operations() {
        // Test create CRUD operations
    }
}
