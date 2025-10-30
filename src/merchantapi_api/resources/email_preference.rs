//! Email_preference resource
//!
//! Returns the email preferences for a Merchant Center account user. This service only permits retrieving and updating email preferences for the authenticated user. Use the name=accounts/*/users/me/emailPreferences alias to get preferences for the authenticated user.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Email_preference resource handler
pub struct Email_preference<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Email_preference<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a email_preference
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a email_preference
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, name: Option<String>, news_and_tips: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_preference_operations() {
        // Test email_preference CRUD operations
    }
}
