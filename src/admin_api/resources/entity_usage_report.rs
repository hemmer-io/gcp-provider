//! Entity_usage_report resource
//!
//! Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entity_usage_report resource handler
pub struct Entity_usage_report<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Entity_usage_report<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entity_usage_report
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
    async fn test_entity_usage_report_operations() {
        // Test entity_usage_report CRUD operations
    }
}
