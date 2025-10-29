//! Trace resource
//!
//! Sends new spans to Stackdriver Trace or updates existing traces. If the
name of a trace that you send matches that of an existing trace, new spans
are added to the existing trace. Attempt to update existing spans results
undefined behavior. If the name does not match, a new trace is created
with given set of spans.

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



    /// Read/describe a trace
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
    async fn test_trace_operations() {
        // Test trace CRUD operations
    }
}
