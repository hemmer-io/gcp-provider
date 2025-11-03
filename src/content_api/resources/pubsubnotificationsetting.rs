//! Pubsubnotificationsetting resource
//!
//! Retrieves a Merchant Center account's pubsub notification settings.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Pubsubnotificationsetting resource handler
pub struct Pubsubnotificationsetting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Pubsubnotificationsetting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a pubsubnotificationsetting
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a pubsubnotificationsetting
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, cloud_topic_name: Option<String>, kind: Option<String>, registered_events: Option<Vec<String>>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pubsubnotificationsetting_operations() {
        // Test pubsubnotificationsetting CRUD operations
    }
}
