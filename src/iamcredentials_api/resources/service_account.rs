//! Service_account resource
//!
//! Signs a JWT using a service account's system-managed private key.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Service_account resource handler
pub struct Service_account<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Service_account<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new service_account
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, delegates: Option<Vec<String>>, payload: Option<String>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_account_operations() {
        // Test service_account CRUD operations
    }
}
