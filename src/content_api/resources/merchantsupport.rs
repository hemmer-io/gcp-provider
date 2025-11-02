//! Merchantsupport resource
//!
//! Start an action. The action can be requested by merchants in third-party application. Before merchants can request the action, the third-party application needs to show them action specific content and display a user input form. The action can be successfully started only once all `required` inputs are provided. If any `required` input is missing, or invalid value was provided, the service will return 400 error. Validation errors will contain Ids for all problematic field together with translated, human readable error messages that can be shown to the user.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Merchantsupport resource handler
pub struct Merchantsupport<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Merchantsupport<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new merchantsupport
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, action_context: Option<String>, action_input: Option<String>, merchant_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merchantsupport_operations() {
        // Test merchantsupport CRUD operations
    }
}
