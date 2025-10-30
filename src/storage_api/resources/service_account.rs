//! Service_account resource
//!
//! Get the email address of this project's Google Cloud Storage service account.

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




    /// Read/describe a service_account
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
    async fn test_service_account_operations() {
        // Test service_account CRUD operations
    }
}
