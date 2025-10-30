//! Runtime_entity_schema resource
//!
//! Lists the JSON schemas for the properties of runtime entities, filtered by entity name.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Runtime_entity_schema resource handler
pub struct Runtime_entity_schema<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Runtime_entity_schema<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a runtime_entity_schema
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
    async fn test_runtime_entity_schema_operations() {
        // Test runtime_entity_schema CRUD operations
    }
}
