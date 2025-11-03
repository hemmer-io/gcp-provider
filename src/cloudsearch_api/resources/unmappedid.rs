//! Unmappedid resource
//!
//! List all unmapped identities for a specific item. **Note:** This API requires an admin account to execute.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Unmappedid resource handler
pub struct Unmappedid<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Unmappedid<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a unmappedid
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
    async fn test_unmappedid_operations() {
        // Test unmappedid CRUD operations
    }
}
