//! Networksecurity Service
//!
//! Auto-generated service module for networksecurity

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networksecurity
pub struct NetworksecurityService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> NetworksecurityService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get intercept_deployment_group resource handler
    pub fn intercept_deployment_group(&self) -> resources::Intercept_deployment_group<'_> {
        resources::Intercept_deployment_group::new(self.provider)
    }
    /// Get authz_policie resource handler
    pub fn authz_policie(&self) -> resources::Authz_policie<'_> {
        resources::Authz_policie::new(self.provider)
    }
    /// Get tls_inspection_policie resource handler
    pub fn tls_inspection_policie(&self) -> resources::Tls_inspection_policie<'_> {
        resources::Tls_inspection_policie::new(self.provider)
    }
    /// Get server_tls_policie resource handler
    pub fn server_tls_policie(&self) -> resources::Server_tls_policie<'_> {
        resources::Server_tls_policie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get authorization_policie resource handler
    pub fn authorization_policie(&self) -> resources::Authorization_policie<'_> {
        resources::Authorization_policie::new(self.provider)
    }
    /// Get sac_attachment resource handler
    pub fn sac_attachment(&self) -> resources::Sac_attachment<'_> {
        resources::Sac_attachment::new(self.provider)
    }
    /// Get gateway_security_policie resource handler
    pub fn gateway_security_policie(&self) -> resources::Gateway_security_policie<'_> {
        resources::Gateway_security_policie::new(self.provider)
    }
    /// Get intercept_endpoint_group resource handler
    pub fn intercept_endpoint_group(&self) -> resources::Intercept_endpoint_group<'_> {
        resources::Intercept_endpoint_group::new(self.provider)
    }
    /// Get mirroring_endpoint_group_association resource handler
    pub fn mirroring_endpoint_group_association(&self) -> resources::Mirroring_endpoint_group_association<'_> {
        resources::Mirroring_endpoint_group_association::new(self.provider)
    }
    /// Get firewall_endpoint resource handler
    pub fn firewall_endpoint(&self) -> resources::Firewall_endpoint<'_> {
        resources::Firewall_endpoint::new(self.provider)
    }
    /// Get mirroring_deployment resource handler
    pub fn mirroring_deployment(&self) -> resources::Mirroring_deployment<'_> {
        resources::Mirroring_deployment::new(self.provider)
    }
    /// Get security_profile resource handler
    pub fn security_profile(&self) -> resources::Security_profile<'_> {
        resources::Security_profile::new(self.provider)
    }
    /// Get address_group resource handler
    pub fn address_group(&self) -> resources::Address_group<'_> {
        resources::Address_group::new(self.provider)
    }
    /// Get mirroring_endpoint_group resource handler
    pub fn mirroring_endpoint_group(&self) -> resources::Mirroring_endpoint_group<'_> {
        resources::Mirroring_endpoint_group::new(self.provider)
    }
    /// Get client_tls_policie resource handler
    pub fn client_tls_policie(&self) -> resources::Client_tls_policie<'_> {
        resources::Client_tls_policie::new(self.provider)
    }
    /// Get backend_authentication_config resource handler
    pub fn backend_authentication_config(&self) -> resources::Backend_authentication_config<'_> {
        resources::Backend_authentication_config::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get firewall_endpoint_association resource handler
    pub fn firewall_endpoint_association(&self) -> resources::Firewall_endpoint_association<'_> {
        resources::Firewall_endpoint_association::new(self.provider)
    }
    /// Get security_profile_group resource handler
    pub fn security_profile_group(&self) -> resources::Security_profile_group<'_> {
        resources::Security_profile_group::new(self.provider)
    }
    /// Get intercept_deployment resource handler
    pub fn intercept_deployment(&self) -> resources::Intercept_deployment<'_> {
        resources::Intercept_deployment::new(self.provider)
    }
    /// Get dns_threat_detector resource handler
    pub fn dns_threat_detector(&self) -> resources::Dns_threat_detector<'_> {
        resources::Dns_threat_detector::new(self.provider)
    }
    /// Get mirroring_deployment_group resource handler
    pub fn mirroring_deployment_group(&self) -> resources::Mirroring_deployment_group<'_> {
        resources::Mirroring_deployment_group::new(self.provider)
    }
    /// Get sac_realm resource handler
    pub fn sac_realm(&self) -> resources::Sac_realm<'_> {
        resources::Sac_realm::new(self.provider)
    }
    /// Get url_list resource handler
    pub fn url_list(&self) -> resources::Url_list<'_> {
        resources::Url_list::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get intercept_endpoint_group_association resource handler
    pub fn intercept_endpoint_group_association(&self) -> resources::Intercept_endpoint_group_association<'_> {
        resources::Intercept_endpoint_group_association::new(self.provider)
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
