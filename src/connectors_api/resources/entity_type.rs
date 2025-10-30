//! Entity_type resource
//!
//! Gets metadata of given entity type

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entity_type resource handler
pub struct Entity_type<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Entity_type<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entity_type
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
    async fn test_entity_type_operations() {
        // Test entity_type CRUD operations
    }
}
