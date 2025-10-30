//! Trace resource
//!
//! Batch writes new spans to new or existing traces. You cannot update existing spans. If a span ID already exists, an additional copy of the span will be stored.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Trace resource handler
pub struct Trace<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Trace<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new trace
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, spans: Option<Vec<String>>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trace_operations() {
        // Test trace CRUD operations
    }
}
