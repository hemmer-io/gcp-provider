//! Permission resource
//!
//! Creates one or more permissions on the note. Only permissions with the `WRITER` role may be created. If adding any permission fails, then the entire request fails and no changes are made.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Permission resource handler
pub struct Permission<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Permission<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new permission
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, requests: Option<Vec<String>>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_permission_operations() {
        // Test permission CRUD operations
    }
}
