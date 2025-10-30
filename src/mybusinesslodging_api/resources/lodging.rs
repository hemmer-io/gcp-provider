//! Lodging resource
//!
//! Returns the Google updated Lodging of a specific location.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Lodging resource handler
pub struct Lodging<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Lodging<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a lodging
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
    async fn test_lodging_operations() {
        // Test lodging CRUD operations
    }
}
