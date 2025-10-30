//! Custom_model resource
//!
//! Gets a list of all the custom models.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Custom_model resource handler
pub struct Custom_model<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Custom_model<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a custom_model
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
    async fn test_custom_model_operations() {
        // Test custom_model CRUD operations
    }
}
