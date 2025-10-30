//! Image_version resource
//!
//! List ImageVersions for provided location.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Image_version resource handler
pub struct Image_version<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Image_version<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a image_version
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
    async fn test_image_version_operations() {
        // Test image_version CRUD operations
    }
}
