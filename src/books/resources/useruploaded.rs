//! Useruploaded resource
//!
//! Return a list of books uploaded by the current user.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Useruploaded resource handler
pub struct Useruploaded<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Useruploaded<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a useruploaded
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
    async fn test_useruploaded_operations() {
        // Test useruploaded CRUD operations
    }
}
