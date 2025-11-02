//! Crypto_key resource
//!
//! Returns cryptographic keys managed by Cloud KMS in a given Cloud project. Note that this data is sourced from snapshots, meaning it may not completely reflect the actual state of key metadata at call time.

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
