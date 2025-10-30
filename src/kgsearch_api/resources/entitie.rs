//! Entitie resource
//!
//! Searches Knowledge Graph for entities that match the constraints. A list of matched entities will be returned in response, which will be in JSON-LD format and compatible with http://schema.org

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entitie resource handler
pub struct Entitie<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Entitie<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entitie
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
    async fn test_entitie_operations() {
        // Test entitie CRUD operations
    }
}
