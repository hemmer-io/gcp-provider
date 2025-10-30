//! Sku resource
//!
//! Lists the SKUs for a product the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Sku resource handler
pub struct Sku<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sku<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a sku
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
    async fn test_sku_operations() {
        // Test sku CRUD operations
    }
}
