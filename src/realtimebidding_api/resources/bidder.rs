//! Bidder resource
//!
//! Gets a bidder account by its name.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Bidder resource handler
pub struct Bidder<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bidder<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a bidder
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
    async fn test_bidder_operations() {
        // Test bidder CRUD operations
    }
}
