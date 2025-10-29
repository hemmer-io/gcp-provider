//! Violating_site resource
//!
//! Lists sites that are failing in the Ad Experience Report on at least one platform.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Violating_site resource handler
pub struct Violating_site<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Violating_site<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a violating_site
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
    async fn test_violating_site_operations() {
        // Test violating_site CRUD operations
    }
}
