//! Hashe resource
//!
//! Search for full hashes matching the specified prefixes. This is a custom method as defined by https://google.aip.dev/136 (the custom method refers to this method having a custom name within Google's general API development nomenclature; it does not refer to using a custom HTTP method).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Hashe resource handler
pub struct Hashe<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Hashe<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a hashe
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
    async fn test_hashe_operations() {
        // Test hashe CRUD operations
    }
}
