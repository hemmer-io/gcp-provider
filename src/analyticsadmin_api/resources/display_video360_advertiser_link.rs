//! Display_video360_advertiser_link resource
//!
//! Creates a DisplayVideo360AdvertiserLink. This can only be utilized by users who have proper authorization both on the Google Analytics property and on the Display & Video 360 advertiser. Users who do not have access to the Display & Video 360 advertiser should instead seek to create a DisplayVideo360LinkProposal.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Display_video360_advertiser_link resource handler
pub struct Display_video360_advertiser_link<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Display_video360_advertiser_link<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new display_video360_advertiser_link
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, ads_personalization_enabled: Option<bool>, advertiser_display_name: Option<String>, cost_data_sharing_enabled: Option<bool>, name: Option<String>, advertiser_id: Option<String>, campaign_data_sharing_enabled: Option<bool>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a display_video360_advertiser_link
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a display_video360_advertiser_link
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, ads_personalization_enabled: Option<bool>, advertiser_display_name: Option<String>, cost_data_sharing_enabled: Option<bool>, name: Option<String>, advertiser_id: Option<String>, campaign_data_sharing_enabled: Option<bool>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a display_video360_advertiser_link
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
    async fn test_display_video360_advertiser_link_operations() {
        // Test display_video360_advertiser_link CRUD operations
    }
}
