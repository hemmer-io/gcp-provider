//! Referencedby resource
//!
//! Get a list of IDs of schemas that reference the schema with the given subject and version.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Referencedby resource handler
pub struct Referencedby<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Referencedby<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a referencedby
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
    async fn test_referencedby_operations() {
        // Test referencedby CRUD operations
    }
}
