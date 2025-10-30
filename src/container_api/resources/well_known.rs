//! Well_known resource
//!
//! Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Well_known resource handler
pub struct Well_known<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Well_known<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a well_known
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
    async fn test_well_known_operations() {
        // Test well_known CRUD operations
    }
}
