//! External_account_key resource
//!
//! Creates a new ExternalAccountKey bound to the project.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// External_account_key resource handler
pub struct External_account_key<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> External_account_key<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new external_account_key
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, b64_mac_key: Option<String>, key_id: Option<String>, name: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_external_account_key_operations() {
        // Test external_account_key CRUD operations
    }
}
