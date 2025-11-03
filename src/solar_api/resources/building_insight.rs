//! Building_insight resource
//!
//! Locates the building whose centroid is closest to a query point. Returns an error with code `NOT_FOUND` if there are no buildings within approximately 50m of the query point.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Building_insight resource handler
pub struct Building_insight<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Building_insight<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a building_insight
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
    async fn test_building_insight_operations() {
        // Test building_insight CRUD operations
    }
}
