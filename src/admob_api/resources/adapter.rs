//! Adapter resource
//!
//! List the adapters of the ad source.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Adapter resource handler
pub struct Adapter<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adapter<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a adapter
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
    async fn test_adapter_operations() {
        // Test adapter CRUD operations
    }
}
