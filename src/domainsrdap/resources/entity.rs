//! Entity resource
//!
//! The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entity resource handler
pub struct Entity<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Entity<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entity
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
    async fn test_entity_operations() {
        // Test entity CRUD operations
    }
}
