//! Iap resource
//!
//! Sets the access control policy for an Identity-Aware Proxy protected resource. Replaces any existing policy. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Iap resource handler
pub struct Iap<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Iap<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new iap
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, policy: Option<String>, resource: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_iap_operations() {
        // Test iap CRUD operations
    }
}
