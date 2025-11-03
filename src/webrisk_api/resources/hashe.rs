//! Hashe resource
//!
//! Gets the full hashes that match the requested hash prefix. This is used after a hash prefix is looked up in a threatList and there is a match. The client side threatList only holds partial hashes so the client must query this method to determine if there is a full hash match of a threat.

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
