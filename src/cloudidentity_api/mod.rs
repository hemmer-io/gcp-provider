//! Cloudidentity_api Service
//!
//! Auto-generated service module for cloudidentity_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudidentity_api
pub struct Cloudidentity_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudidentity_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get userinvitation resource handler
    pub fn userinvitation(&self) -> resources::Userinvitation<'_> {
        resources::Userinvitation::new(self.provider)
    }
    /// Get inbound_oidc_sso_profile resource handler
    pub fn inbound_oidc_sso_profile(&self) -> resources::Inbound_oidc_sso_profile<'_> {
        resources::Inbound_oidc_sso_profile::new(self.provider)
    }
    /// Get device_user resource handler
    pub fn device_user(&self) -> resources::Device_user<'_> {
        resources::Device_user::new(self.provider)
    }
    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get idp_credential resource handler
    pub fn idp_credential(&self) -> resources::Idp_credential<'_> {
        resources::Idp_credential::new(self.provider)
    }
    /// Get client_state resource handler
    pub fn client_state(&self) -> resources::Client_state<'_> {
        resources::Client_state::new(self.provider)
    }
    /// Get inbound_saml_sso_profile resource handler
    pub fn inbound_saml_sso_profile(&self) -> resources::Inbound_saml_sso_profile<'_> {
        resources::Inbound_saml_sso_profile::new(self.provider)
    }
    /// Get membership resource handler
    pub fn membership(&self) -> resources::Membership<'_> {
        resources::Membership::new(self.provider)
    }
    /// Get inbound_sso_assignment resource handler
    pub fn inbound_sso_assignment(&self) -> resources::Inbound_sso_assignment<'_> {
        resources::Inbound_sso_assignment::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
