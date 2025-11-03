//! Custom_data_source resource
//!
//! List custom data sources to which the user has access.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Custom_data_source resource handler
pub struct Custom_data_source<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Custom_data_source<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a custom_data_source
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
    async fn test_custom_data_source_operations() {
        // Test custom_data_source CRUD operations
    }
}
