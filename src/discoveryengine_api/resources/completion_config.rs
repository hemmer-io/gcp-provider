//! Completion_config resource
//!
//! Removes the search history suggestion in an engine for a user. This will remove the suggestion from being returned in the AdvancedCompleteQueryResponse.recent_search_suggestions for this user. If the user searches the same suggestion again, the new history will override and suggest this suggestion again.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Completion_config resource handler
pub struct Completion_config<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Completion_config<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new completion_config
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, user_info: Option<String>, search_history_suggestion: Option<String>, user_pseudo_id: Option<String>, remove_all_search_history_suggestions: Option<bool>, remove_time: Option<String>, completion_config: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_completion_config_operations() {
        // Test completion_config CRUD operations
    }
}
