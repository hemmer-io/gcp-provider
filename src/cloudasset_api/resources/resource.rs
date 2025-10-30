//! Resource resource
//!
//! Searches all the resources within a given accessible Resource Manager scope (project/folder/organization). This RPC gives callers especially administrators the ability to search all the resources within a scope, even if they don't have `.get` permission of all the resources. Callers should have `cloudasset.assets.searchAllResources` permission on the requested scope, otherwise the request will be rejected.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Resource resource handler
pub struct Resource<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Resource<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a resource
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
    async fn test_resource_operations() {
        // Test resource CRUD operations
    }
}
