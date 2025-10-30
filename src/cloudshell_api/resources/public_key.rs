//! Public_key resource
//!
//! Adds a public SSH key to an environment, allowing clients with the corresponding private key to connect to that environment via SSH. If a key with the same format and content already exists, this will return the existing key.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Public_key resource handler
pub struct Public_key<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Public_key<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new public_key
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, key: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







    /// Delete a public_key
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_public_key_operations() {
        // Test public_key CRUD operations
    }
}
