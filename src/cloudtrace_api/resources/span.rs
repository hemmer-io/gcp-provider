//! Span resource
//!
//! Creates a new span. If a span ID already exists, an additional copy of the span will be stored.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Span resource handler
pub struct Span<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Span<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new span
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, child_span_count: Option<i64>, status: Option<String>, parent_span_id: Option<String>, attributes: Option<String>, end_time: Option<String>, links: Option<String>, same_process_as_parent_span: Option<bool>, span_kind: Option<String>, stack_trace: Option<String>, start_time: Option<String>, time_events: Option<String>, name: Option<String>, display_name: Option<String>, span_id: Option<String>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_span_operations() {
        // Test span CRUD operations
    }
}
