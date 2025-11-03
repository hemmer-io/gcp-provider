//! Cloudsearch resource
//!
//! Enables `third party` support in Google Cloud Search. **Note:** This API requires an admin account to execute.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Cloudsearch resource handler
pub struct Cloudsearch<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudsearch<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new cloudsearch
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloudsearch_operations() {
        // Test cloudsearch CRUD operations
    }
}
