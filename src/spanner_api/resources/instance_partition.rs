//! Instance_partition resource
//!
//! Creates an instance partition and begins preparing it to be used. The returned long-running operation can be used to track the progress of preparing the new instance partition. The instance partition name is assigned by the caller. If the named instance partition already exists, `CreateInstancePartition` returns `ALREADY_EXISTS`. Immediately upon completion of this request: * The instance partition is readable via the API, with all requested attributes but no allocated resources. Its state is `CREATING`. Until completion of the returned operation: * Cancelling the operation renders the instance partition immediately unreadable via the API. * The instance partition can be deleted. * All other attempts to modify the instance partition are rejected. Upon completion of the returned operation: * Billing for all successfully-allocated resources begins (some types may have lower than the requested levels). * Databases can start using this instance partition. * The instance partition's allocated resource levels are readable via the API. * The instance partition's state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance partition. The metadata field type is CreateInstancePartitionMetadata. The response field type is InstancePartition, if successful.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Instance_partition resource handler
pub struct Instance_partition<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Instance_partition<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new instance_partition
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, instance_partition: Option<String>, instance_partition_id: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a instance_partition
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a instance_partition
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, instance_partition: Option<String>, instance_partition_id: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a instance_partition
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_instance_partition_operations() {
        // Test instance_partition CRUD operations
    }
}
