//! First_party_and_partner_audience resource
//!
//! Creates a FirstPartyAndPartnerAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// First_party_and_partner_audience resource handler
pub struct First_party_and_partner_audience<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> First_party_and_partner_audience<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new first_party_and_partner_audience
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, audience_source: Option<String>, contact_info_list: Option<String>, audience_type: Option<String>, membership_duration_days: Option<String>, name: Option<String>, first_party_and_partner_audience_type: Option<String>, display_mobile_app_audience_size: Option<String>, gmail_audience_size: Option<String>, youtube_audience_size: Option<String>, first_party_and_partner_audience_id: Option<String>, description: Option<String>, display_audience_size: Option<String>, active_display_audience_size: Option<String>, mobile_device_id_list: Option<String>, display_desktop_audience_size: Option<String>, display_mobile_web_audience_size: Option<String>, display_name: Option<String>, app_id: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a first_party_and_partner_audience
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a first_party_and_partner_audience
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, audience_source: Option<String>, contact_info_list: Option<String>, audience_type: Option<String>, membership_duration_days: Option<String>, name: Option<String>, first_party_and_partner_audience_type: Option<String>, display_mobile_app_audience_size: Option<String>, gmail_audience_size: Option<String>, youtube_audience_size: Option<String>, first_party_and_partner_audience_id: Option<String>, description: Option<String>, display_audience_size: Option<String>, active_display_audience_size: Option<String>, mobile_device_id_list: Option<String>, display_desktop_audience_size: Option<String>, display_mobile_web_audience_size: Option<String>, display_name: Option<String>, app_id: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_first_party_and_partner_audience_operations() {
        // Test first_party_and_partner_audience CRUD operations
    }
}
