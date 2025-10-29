//! Beyondcorp Service
//!
//! Auto-generated service module for beyondcorp

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for beyondcorp
pub struct BeyondcorpService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BeyondcorpService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get partner_tenant resource handler
    pub fn partner_tenant(&self) -> resources::Partner_tenant<'_> {
        resources::Partner_tenant::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get app_connection resource handler
    pub fn app_connection(&self) -> resources::App_connection<'_> {
        resources::App_connection::new(self.provider)
    }
    /// Get app_gateway resource handler
    pub fn app_gateway(&self) -> resources::App_gateway<'_> {
        resources::App_gateway::new(self.provider)
    }
    /// Get application_domain resource handler
    pub fn application_domain(&self) -> resources::Application_domain<'_> {
        resources::Application_domain::new(self.provider)
    }
    /// Get app_connector resource handler
    pub fn app_connector(&self) -> resources::App_connector<'_> {
        resources::App_connector::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get security_gateway resource handler
    pub fn security_gateway(&self) -> resources::Security_gateway<'_> {
        resources::Security_gateway::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
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
