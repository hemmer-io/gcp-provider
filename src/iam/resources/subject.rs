//! Subject resource
//!
//! Undeletes a WorkforcePoolSubject, as long as it was deleted fewer than 30 days ago.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Subject resource handler
pub struct Subject<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Subject<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new subject
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







    /// Delete a subject
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subject_operations() {
        // Test subject CRUD operations
    }
}
