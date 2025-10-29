//! Ga resource
//!
//! Returns Analytics data for a view (profile).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Ga resource handler
pub struct Ga<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Ga<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a ga
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
    async fn test_ga_operations() {
        // Test ga CRUD operations
    }
}
