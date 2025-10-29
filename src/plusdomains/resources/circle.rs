//! Circle resource
//!
//! Shut down. See https://developers.google.com/+/api-shutdown for more details.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Circle resource handler
pub struct Circle<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Circle<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a circle
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
    async fn test_circle_operations() {
        // Test circle CRUD operations
    }
}
