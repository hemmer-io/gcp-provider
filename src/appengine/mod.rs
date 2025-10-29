//! Appengine Service
//!
//! Auto-generated service module for appengine

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appengine
pub struct AppengineService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AppengineService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get authorized_domain resource handler
    pub fn authorized_domain(&self) -> resources::Authorized_domain<'_> {
        resources::Authorized_domain::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get authorized_certificate resource handler
    pub fn authorized_certificate(&self) -> resources::Authorized_certificate<'_> {
        resources::Authorized_certificate::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get domain_mapping resource handler
    pub fn domain_mapping(&self) -> resources::Domain_mapping<'_> {
        resources::Domain_mapping::new(self.provider)
    }
    /// Get ingress_rule resource handler
    pub fn ingress_rule(&self) -> resources::Ingress_rule<'_> {
        resources::Ingress_rule::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
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
