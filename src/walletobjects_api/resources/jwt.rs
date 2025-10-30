//! Jwt resource
//!
//! Inserts the resources in the JWT.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Jwt resource handler
pub struct Jwt<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Jwt<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new jwt
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, jwt: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt_operations() {
        // Test jwt CRUD operations
    }
}
