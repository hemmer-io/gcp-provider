//! Issuer resource
//!
//! Lists hybrid services and its trusted issuers service account ids. This api is authenticated and unauthorized(allow all the users) and used by runtime authn-authz service to query control plane's issuer service account ids.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Issuer resource handler
pub struct Issuer<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Issuer<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a issuer
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
    async fn test_issuer_operations() {
        // Test issuer CRUD operations
    }
}
