//! Instance_partition_operation resource
//!
//! Lists instance partition long-running operations in the given instance. An instance partition operation has a name of the form `projects//instances//instancePartitions//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation. Authorization requires `spanner.instancePartitionOperations.list` permission on the resource parent.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Instance_partition_operation resource handler
pub struct Instance_partition_operation<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Instance_partition_operation<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a instance_partition_operation
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
    async fn test_instance_partition_operation_operations() {
        // Test instance_partition_operation CRUD operations
    }
}
