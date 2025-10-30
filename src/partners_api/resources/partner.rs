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
    pub async fn update(&self, id: &str, website_url: Option<String>, id: Option<String>, adwords_customer_id: Option<String>, min_monthly_budget: Option<String>, create_time: Option<String>, comments: Option<String>, email: Option<String>, family_name: Option<String>, marketing_opt_in: Option<bool>, phone_number: Option<String>, type: Option<String>, language_code: Option<String>, given_name: Option<String>, state: Option<String>, gps_motivations: Option<Vec<String>>) -> Result<()> {

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
