//! Cloudloading resource
//!
//! Remove the book and its contents

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Cloudloading resource handler
pub struct Cloudloading<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudloading<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new cloudloading
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
    async fn test_cloudloading_operations() {
        // Test cloudloading CRUD operations
    }
}
