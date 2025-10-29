//! Ssh_key resource
//!
//! Register a public SSH key in the specified project for use with the interactive serial console feature.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Ssh_key resource handler
pub struct Ssh_key<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Ssh_key<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new ssh_key
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, name: Option<String>, public_key: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a ssh_key
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





    /// Delete a ssh_key
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
    async fn test_ssh_key_operations() {
        // Test ssh_key CRUD operations
    }
}
