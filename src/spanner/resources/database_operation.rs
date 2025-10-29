//! Database_operation resource
//!
//! Lists database longrunning-operations. A database operation has a name of the form `projects//instances//databases//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Database_operation resource handler
pub struct Database_operation<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Database_operation<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a database_operation
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
    async fn test_database_operation_operations() {
        // Test database_operation CRUD operations
    }
}
