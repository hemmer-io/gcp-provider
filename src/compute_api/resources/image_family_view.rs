//! Image_family_view resource
//!
//! Returns the latest image that is part of an image family, is not
deprecated and is rolled out in the specified zone.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Image_family_view resource handler
pub struct Image_family_view<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Image_family_view<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a image_family_view
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
    async fn test_image_family_view_operations() {
        // Test image_family_view CRUD operations
    }
}
