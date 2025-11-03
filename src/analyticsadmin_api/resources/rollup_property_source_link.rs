//! Rollup_property_source_link resource
//!
//! Creates a roll-up property source link. Only roll-up properties can have source links, so this method will throw an error if used on other types of properties.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Rollup_property_source_link resource handler
pub struct Rollup_property_source_link<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Rollup_property_source_link<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new rollup_property_source_link
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, name: Option<String>, source_property: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a rollup_property_source_link
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





    /// Delete a rollup_property_source_link
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rollup_property_source_link_operations() {
        // Test rollup_property_source_link CRUD operations
    }
}
