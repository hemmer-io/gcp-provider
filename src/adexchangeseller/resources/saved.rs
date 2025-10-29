//! Saved resource
//!
//! List all saved reports in this Ad Exchange account.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Saved resource handler
pub struct Saved<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Saved<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a saved
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
    async fn test_saved_operations() {
        // Test saved CRUD operations
    }
}
