//! Customer resource
//!
//! Orders a new customer's account. Before ordering a new customer account, establish whether the customer account already exists using the [`customers.get`](https://developers.google.com/workspace/admin/reseller/v1/reference/customers/get) If the customer account exists as a direct Google account or as a resold customer account from another reseller, use the `customerAuthToken\` as described in [order a resold account for an existing customer](https://developers.google.com/workspace/admin/reseller/v1/how-tos/manage_customers#create_existing_customer). For more information about ordering a new customer account, see [order a new customer account](https://developers.google.com/workspace/admin/reseller/v1/how-tos/manage_customers#create_customer). After creating a new customer account, you must provision a user as an administrator. The customer's administrator is required to sign in to the Admin console and sign the G Suite via Reseller agreement to activate the account. Resellers are prohibited from signing the G Suite via Reseller agreement on the customer's behalf. For more information, see [order a new customer account](https://developers.google.com/workspace/admin/reseller/v1/how-tos/manage_customers#tos).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Customer resource handler
pub struct Customer<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Customer<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new customer
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, customer_domain_verified: Option<bool>, customer_id: Option<String>, customer_domain: Option<String>, primary_admin: Option<String>, alternate_email: Option<String>, postal_address: Option<String>, resource_ui_url: Option<String>, kind: Option<String>, customer_type: Option<String>, phone_number: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a customer
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a customer
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, customer_domain_verified: Option<bool>, customer_id: Option<String>, customer_domain: Option<String>, primary_admin: Option<String>, alternate_email: Option<String>, postal_address: Option<String>, resource_ui_url: Option<String>, kind: Option<String>, customer_type: Option<String>, phone_number: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_customer_operations() {
        // Test customer CRUD operations
    }
}
