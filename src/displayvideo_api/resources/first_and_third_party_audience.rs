//! First_and_third_party_audience resource
//!
//! Creates a FirstAndThirdPartyAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// First_and_third_party_audience resource handler
pub struct First_and_third_party_audience<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> First_and_third_party_audience<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new first_and_third_party_audience
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, app_id: Option<String>, audience_type: Option<String>, display_audience_size: Option<String>, display_mobile_app_audience_size: Option<String>, first_and_third_party_audience_type: Option<String>, description: Option<String>, display_desktop_audience_size: Option<String>, gmail_audience_size: Option<String>, membership_duration_days: Option<String>, youtube_audience_size: Option<String>, contact_info_list: Option<String>, display_name: Option<String>, audience_source: Option<String>, mobile_device_id_list: Option<String>, name: Option<String>, first_and_third_party_audience_id: Option<String>, active_display_audience_size: Option<String>, display_mobile_web_audience_size: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a first_and_third_party_audience
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a first_and_third_party_audience
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, app_id: Option<String>, audience_type: Option<String>, display_audience_size: Option<String>, display_mobile_app_audience_size: Option<String>, first_and_third_party_audience_type: Option<String>, description: Option<String>, display_desktop_audience_size: Option<String>, gmail_audience_size: Option<String>, membership_duration_days: Option<String>, youtube_audience_size: Option<String>, contact_info_list: Option<String>, display_name: Option<String>, audience_source: Option<String>, mobile_device_id_list: Option<String>, name: Option<String>, first_and_third_party_audience_id: Option<String>, active_display_audience_size: Option<String>, display_mobile_web_audience_size: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_first_and_third_party_audience_operations() {
        // Test first_and_third_party_audience CRUD operations
    }
}
