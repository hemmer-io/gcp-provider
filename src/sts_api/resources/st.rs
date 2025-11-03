//! St resource
//!
//! Exchanges a credential for a Google OAuth 2.0 access token. The token asserts an external identity within a workload identity pool, or it applies a Credential Access Boundary to a Google access token. When you call this method, do not send the `Authorization` HTTP header in the request. This method does not require the `Authorization` header, and using the header can cause the request to fail.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// St resource handler
pub struct St<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> St<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new st
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, options: Option<String>, scope: Option<String>, requested_token_type: Option<String>, grant_type: Option<String>, subject_token_type: Option<String>, subject_token: Option<String>, audience: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_st_operations() {
        // Test st CRUD operations
    }
}
