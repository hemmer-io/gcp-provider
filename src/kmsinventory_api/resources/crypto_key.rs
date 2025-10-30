//! Crypto_key resource
//!
//! Returns aggregate information about the resources protected by the given Cloud KMS CryptoKey. Only resources within the same Cloud organization as the key will be returned. The project that holds the key must be part of an organization in order for this call to succeed.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Crypto_key resource handler
pub struct Crypto_key<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Crypto_key<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a crypto_key
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
    async fn test_crypto_key_operations() {
        // Test crypto_key CRUD operations
    }
}
