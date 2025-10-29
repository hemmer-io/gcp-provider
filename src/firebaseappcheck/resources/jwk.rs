//! Jwk resource
//!
//! Returns a public JWK set as specified by [RFC 7517](https://tools.ietf.org/html/rfc7517) that can be used to verify App Check tokens. Exactly one of the public keys in the returned set will successfully validate any App Check token that is currently valid.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Jwk resource handler
pub struct Jwk<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Jwk<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a jwk
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
    async fn test_jwk_operations() {
        // Test jwk CRUD operations
    }
}
