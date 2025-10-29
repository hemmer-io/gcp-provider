//! Partner resource
//!
//! Gets Partners Status of the logged in user's agency.
Should only be called if the logged in user is the admin of the agency.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Partner resource handler
pub struct Partner<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Partner<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a partner
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a partner
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, ranks: Option<Vec<String>>, primary_adwords_manager_account_id: Option<String>, additional_websites: Option<Vec<String>>, auto_approval_email_domains: Option<Vec<String>>, original_min_monthly_budget: Option<String>, profile_status: Option<String>, specialization_status: Option<Vec<String>>, certification_statuses: Option<Vec<String>>, services: Option<Vec<String>>, website_url: Option<String>, converted_min_monthly_budget: Option<String>, locations: Option<Vec<String>>, badge_tier: Option<String>, localized_infos: Option<Vec<String>>, name: Option<String>, public_profile: Option<String>, primary_location: Option<String>, industries: Option<Vec<String>>, badge_authority_in_awn: Option<bool>, company_types: Option<Vec<String>>, id: Option<String>, primary_language_code: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_partner_operations() {
        // Test partner CRUD operations
    }
}
