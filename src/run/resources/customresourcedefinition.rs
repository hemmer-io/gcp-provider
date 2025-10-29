//! Customresourcedefinition resource
//!
//! Rpc to get information about a CustomResourceDefinition.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Customresourcedefinition resource handler
pub struct Customresourcedefinition<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Customresourcedefinition<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a customresourcedefinition
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
    async fn test_customresourcedefinition_operations() {
        // Test customresourcedefinition CRUD operations
    }
}
