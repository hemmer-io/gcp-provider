//! Disk_setting resource
//!
//! Get Zonal Disk Settings.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Disk_setting resource handler
pub struct Disk_setting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Disk_setting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a disk_setting
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a disk_setting
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, access_location: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_disk_setting_operations() {
        // Test disk_setting CRUD operations
    }
}
