//! Well-known resource
//!
//! Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Well-known resource handler
pub struct Well-known<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Well-known<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a well-known
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
    async fn test_well-known_operations() {
        // Test well-known CRUD operations
    }
}
