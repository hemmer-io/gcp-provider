//! Disk_type resource
//!
//! Returns the specified disk type.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Disk_type resource handler
pub struct Disk_type<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Disk_type<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a disk_type
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
    async fn test_disk_type_operations() {
        // Test disk_type CRUD operations
    }
}
