//! Preferreddeal resource
//!
//! Get information about the selected Ad Exchange Preferred Deal.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Preferreddeal resource handler
pub struct Preferreddeal<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Preferreddeal<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a preferreddeal
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
    async fn test_preferreddeal_operations() {
        // Test preferreddeal CRUD operations
    }
}
