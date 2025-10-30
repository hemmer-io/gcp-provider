//! Instance_config_operation resource
//!
//! Lists the user-managed instance configuration long-running operations in the given project. An instance configuration operation has a name of the form `projects//instanceConfigs//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Instance_config_operation resource handler
pub struct Instance_config_operation<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Instance_config_operation<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a instance_config_operation
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
    async fn test_instance_config_operation_operations() {
        // Test instance_config_operation CRUD operations
    }
}
