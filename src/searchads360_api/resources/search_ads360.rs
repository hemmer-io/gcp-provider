//! Search_ads360 resource
//!
//! Returns all rows that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Search_ads360 resource handler
pub struct Search_ads360<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Search_ads360<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new search_ads360
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, return_total_results_count: Option<bool>, summary_row_setting: Option<String>, page_token: Option<String>, query: Option<String>, validate_only: Option<bool>, page_size: Option<i64>, customer_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_ads360_operations() {
        // Test search_ads360 CRUD operations
    }
}
