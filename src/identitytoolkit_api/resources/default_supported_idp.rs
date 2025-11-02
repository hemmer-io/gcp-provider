//! Default_supported_idp resource
//!
//! List all default supported Idps.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Default_supported_idp resource handler
pub struct Default_supported_idp<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Default_supported_idp<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a default_supported_idp
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
    async fn test_default_supported_idp_operations() {
        // Test default_supported_idp CRUD operations
    }
}
