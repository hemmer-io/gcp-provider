//! Log_service resource
//!
//! Lists log services associated with log entries ingested for a project.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Log_service resource handler
pub struct Log_service<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Log_service<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a log_service
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
    async fn test_log_service_operations() {
        // Test log_service CRUD operations
    }
}
