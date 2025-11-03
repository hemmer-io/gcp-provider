//! User_usage_report resource
//!
//! Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// User_usage_report resource handler
pub struct User_usage_report<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> User_usage_report<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a user_usage_report
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_usage_report_operations() {
        // Test user_usage_report CRUD operations
    }
}
