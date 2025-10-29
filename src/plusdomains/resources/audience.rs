//! Audience resource
//!
//! Shut down. See https://developers.google.com/+/api-shutdown for more details.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Audience resource handler
pub struct Audience<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Audience<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a audience
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
    async fn test_audience_operations() {
        // Test audience CRUD operations
    }
}
