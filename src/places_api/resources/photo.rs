//! Photo resource
//!
//! Get a photo media with a photo reference string.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Photo resource handler
pub struct Photo<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Photo<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a photo
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
    async fn test_photo_operations() {
        // Test photo CRUD operations
    }
}
