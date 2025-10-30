//! Go_module resource
//!
//! Directly uploads a Go module. The returned Operation will complete once the Go module is uploaded. Package, Version, and File resources are created based on the uploaded Go module.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Go_module resource handler
pub struct Go_module<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Go_module<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new go_module
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_go_module_operations() {
        // Test go_module CRUD operations
    }
}
