//! Associationsession resource
//!
//! Create an association session for initiating an association with an AdSense user.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Associationsession resource handler
pub struct Associationsession<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Associationsession<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a associationsession
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
    async fn test_associationsession_operations() {
        // Test associationsession CRUD operations
    }
}
