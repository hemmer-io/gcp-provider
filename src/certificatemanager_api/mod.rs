//! Certificatemanager_api Service
//!
//! Auto-generated service module for certificatemanager_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for certificatemanager_api
pub struct Certificatemanager_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Certificatemanager_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get certificate_issuance_config resource handler
    pub fn certificate_issuance_config(&self) -> resources::Certificate_issuance_config<'_> {
        resources::Certificate_issuance_config::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get dns_authorization resource handler
    pub fn dns_authorization(&self) -> resources::Dns_authorization<'_> {
        resources::Dns_authorization::new(self.provider)
    }
    /// Get trust_config resource handler
    pub fn trust_config(&self) -> resources::Trust_config<'_> {
        resources::Trust_config::new(self.provider)
    }
    /// Get certificate_map resource handler
    pub fn certificate_map(&self) -> resources::Certificate_map<'_> {
        resources::Certificate_map::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get certificate_map_entrie resource handler
    pub fn certificate_map_entrie(&self) -> resources::Certificate_map_entrie<'_> {
        resources::Certificate_map_entrie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
