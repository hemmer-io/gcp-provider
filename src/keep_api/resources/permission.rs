//! Permission resource
//!
//! Deletes one or more permissions on the note. The specified entities will immediately lose access. A permission with the `OWNER` role can't be removed. If removing a permission fails, then the entire request fails and no changes are made. Returns a 400 bad request error if a specified permission does not exist on the note.

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
    pub async fn create(&self, names: Option<Vec<String>>, parent: String) -> Result<String> {

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
