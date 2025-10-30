//! Scan_run resource
//!
//! Stops a ScanRun. The stopped ScanRun is returned.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Scan_run resource handler
pub struct Scan_run<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Scan_run<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new scan_run
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a scan_run
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
    async fn test_scan_run_operations() {
        // Test scan_run CRUD operations
    }
}
