//! Location resource
//!
//! Returns the Lodging of a specific location.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Location resource handler
pub struct Location<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Location<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a location
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a location
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, health_and_safety: Option<String>, pools: Option<String>, transportation: Option<String>, sustainability: Option<String>, common_living_area: Option<String>, business: Option<String>, all_units: Option<String>, accessibility: Option<String>, connectivity: Option<String>, services: Option<String>, some_units: Option<String>, name: Option<String>, activities: Option<String>, pets: Option<String>, wellness: Option<String>, parking: Option<String>, metadata: Option<String>, food_and_drink: Option<String>, housekeeping: Option<String>, guest_units: Option<Vec<String>>, property: Option<String>, policies: Option<String>, families: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_location_operations() {
        // Test location CRUD operations
    }
}
