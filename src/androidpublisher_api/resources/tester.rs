//! Tester resource
//!
//! Gets testers. Note: Testers resource does not support email lists.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Tester resource handler
pub struct Tester<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Tester<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a tester
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a tester
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, google_groups: Option<Vec<String>>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tester_operations() {
        // Test tester CRUD operations
    }
}
