//! People resource
//!
//! Get a person's profile.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// People resource handler
pub struct People<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> People<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a people
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
    async fn test_people_operations() {
        // Test people CRUD operations
    }
}
