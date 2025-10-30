//! Vendor resource
//!
//! Lists the vendors of the partner.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Vendor resource handler
pub struct Vendor<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Vendor<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a vendor
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
    async fn test_vendor_operations() {
        // Test vendor CRUD operations
    }
}
