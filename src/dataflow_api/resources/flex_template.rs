//! Flex_template resource
//!
//! Launch a job with a FlexTemplate.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Flex_template resource handler
pub struct Flex_template<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Flex_template<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new flex_template
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, validate_only: Option<bool>, launch_parameter: Option<String>, project_id: String, location: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flex_template_operations() {
        // Test flex_template CRUD operations
    }
}
