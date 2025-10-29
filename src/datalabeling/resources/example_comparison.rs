//! Example_comparison resource
//!
//! Searches example comparisons from an evaluation. The return format is a list of example comparisons that show ground truth and prediction(s) for a single input. Search by providing an evaluation ID.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Example_comparison resource handler
pub struct Example_comparison<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Example_comparison<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new example_comparison
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, page_size: Option<i64>, page_token: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_comparison_operations() {
        // Test example_comparison CRUD operations
    }
}
