//! Google_location resource
//!
//! Search all of the possible locations that are a match to the specified request.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Google_location resource handler
pub struct Google_location<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Google_location<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new google_location
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, page_size: Option<i64>, query: Option<String>, location: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google_location_operations() {
        // Test google_location CRUD operations
    }
}
