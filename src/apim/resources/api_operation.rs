//! Api_operation resource
//!
//! GetApiOperation retrieves a single ApiOperation by name.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Api_operation resource handler
pub struct Api_operation<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Api_operation<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a api_operation
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
    async fn test_api_operation_operations() {
        // Test api_operation CRUD operations
    }
}
