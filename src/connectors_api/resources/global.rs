//! Global resource
//!
//! GetGlobalSettings gets settings of a project. GlobalSettings is a singleton resource.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Global resource handler
pub struct Global<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Global<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a global
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a global
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, tenant_project_id: Option<String>, name: Option<String>, vpcsc: Option<bool>, payg: Option<bool>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_global_operations() {
        // Test global CRUD operations
    }
}
