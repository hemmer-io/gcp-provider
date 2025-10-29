//! Completion_suggestion resource
//!
//! Imports CompletionSuggestions for a DataStore.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Completion_suggestion resource handler
pub struct Completion_suggestion<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Completion_suggestion<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new completion_suggestion
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, error_config: Option<String>, inline_source: Option<String>, gcs_source: Option<String>, bigquery_source: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_completion_suggestion_operations() {
        // Test completion_suggestion CRUD operations
    }
}
