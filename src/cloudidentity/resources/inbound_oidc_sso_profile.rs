//! Inbound_oidc_sso_profile resource
//!
//! Creates an InboundOidcSsoProfile for a customer. When the target customer has enabled [Multi-party approval for sensitive actions](https://support.google.com/a/answer/13790448), the `Operation` in the response will have `"done": false`, it will not have a response, and the metadata will have `"state": "awaiting-multi-party-approval"`.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Inbound_oidc_sso_profile resource handler
pub struct Inbound_oidc_sso_profile<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Inbound_oidc_sso_profile<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new inbound_oidc_sso_profile
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, idp_config: Option<String>, rp_config: Option<String>, name: Option<String>, display_name: Option<String>, customer: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a inbound_oidc_sso_profile
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a inbound_oidc_sso_profile
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, idp_config: Option<String>, rp_config: Option<String>, name: Option<String>, display_name: Option<String>, customer: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a inbound_oidc_sso_profile
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
    async fn test_inbound_oidc_sso_profile_operations() {
        // Test inbound_oidc_sso_profile CRUD operations
    }
}
