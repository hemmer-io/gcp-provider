//! Peering_route resource
//!
//! Lists the private connection routes exchanged over a peering connection.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Peering_route resource handler
pub struct Peering_route<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Peering_route<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a peering_route
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
    async fn test_peering_route_operations() {
        // Test peering_route CRUD operations
    }
}
