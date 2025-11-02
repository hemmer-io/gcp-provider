//! Suggestion resource
//!
//! Suggests summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Suggestion resource handler
pub struct Suggestion<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Suggestion<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new suggestion
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, assist_query_params: Option<String>, context_size: Option<i64>, latest_message: Option<String>, conversation: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_suggestion_operations() {
        // Test suggestion CRUD operations
    }
}
