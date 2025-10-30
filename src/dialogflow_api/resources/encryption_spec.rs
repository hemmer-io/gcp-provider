//! Encryption_spec resource
//!
//! Initializes a location-level encryption key specification. An error will be thrown if the location has resources already created before the initialization. Once the encryption specification is initialized at a location, it is immutable and all newly created resources under the location will be encrypted with the existing specification.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Encryption_spec resource handler
pub struct Encryption_spec<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Encryption_spec<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new encryption_spec
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, encryption_spec: Option<String>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption_spec_operations() {
        // Test encryption_spec CRUD operations
    }
}
