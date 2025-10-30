//! Backup_run resource
//!
//! Creates a new backup run on demand. This method is applicable only to
Second Generation instances.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Backup_run resource handler
pub struct Backup_run<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Backup_run<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new backup_run
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, end_time: Option<String>, description: Option<String>, instance: Option<String>, status: Option<String>, self_link: Option<String>, enqueued_time: Option<String>, error: Option<String>, location: Option<String>, window_start_time: Option<String>, kind: Option<String>, disk_encryption_status: Option<String>, disk_encryption_configuration: Option<String>, start_time: Option<String>, type: Option<String>, id: Option<String>, project: String, instance: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a backup_run
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





    /// Delete a backup_run
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
    async fn test_backup_run_operations() {
        // Test backup_run CRUD operations
    }
}
