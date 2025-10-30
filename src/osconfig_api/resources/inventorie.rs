//! Inventorie resource
//!
//! Get inventory data for the specified VM instance. If the VM has no associated inventory, the message `NOT_FOUND` is returned.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Inventorie resource handler
pub struct Inventorie<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Inventorie<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a inventorie
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
    async fn test_inventorie_operations() {
        // Test inventorie CRUD operations
    }
}
