//! Discovered_api_operation resource
//!
//! Gets a DiscoveredAPIOperation in a given project, location, ApiObservation and ApiOperation.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Discovered_api_operation resource handler
pub struct Discovered_api_operation<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Discovered_api_operation<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a discovered_api_operation
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
    async fn test_discovered_api_operation_operations() {
        // Test discovered_api_operation CRUD operations
    }
}
