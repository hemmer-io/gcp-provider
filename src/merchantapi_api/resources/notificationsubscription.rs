//! Notificationsubscription resource
//!
//! Creates a notification subscription for a business. For standalone or subaccounts accounts, the business can create a subscription for self. For MCAs, the business can create a subscription for all managed accounts or for a specific subaccount. We will allow the following types of notification subscriptions to exist together (per business as a subscriber per event type): 1. Subscription for all managed accounts + subscription for self. 2. Multiple "partial" subscriptions for managed accounts + subscription for self. we will not allow (per business as a subscriber per event type): 1. Multiple self subscriptions. 2. Multiple "all managed accounts" subscriptions. 3. "All managed accounts" subscription and partial subscriptions at the same time. 4. Multiple partial subscriptions for the same target account.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Notificationsubscription resource handler
pub struct Notificationsubscription<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Notificationsubscription<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new notificationsubscription
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, all_managed_accounts: Option<bool>, name: Option<String>, call_back_uri: Option<String>, registered_event: Option<String>, target_account: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a notificationsubscription
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a notificationsubscription
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, all_managed_accounts: Option<bool>, name: Option<String>, call_back_uri: Option<String>, registered_event: Option<String>, target_account: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a notificationsubscription
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
    async fn test_notificationsubscription_operations() {
        // Test notificationsubscription CRUD operations
    }
}
