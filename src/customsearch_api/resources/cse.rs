//! Cse resource
//!
//! Returns metadata about the search performed, metadata about the engine used for the search, and the search results.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Cse resource handler
pub struct Cse<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cse<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a cse
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
    async fn test_cse_operations() {
        // Test cse CRUD operations
    }
}
