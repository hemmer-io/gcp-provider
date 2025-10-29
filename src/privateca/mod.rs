//! Privateca Service
//!
//! Auto-generated service module for privateca

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for privateca
pub struct PrivatecaService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PrivatecaService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get certificate_revocation_list resource handler
    pub fn certificate_revocation_list(&self) -> resources::Certificate_revocation_list<'_> {
        resources::Certificate_revocation_list::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get certificate_authoritie resource handler
    pub fn certificate_authoritie(&self) -> resources::Certificate_authoritie<'_> {
        resources::Certificate_authoritie::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get reusable_config resource handler
    pub fn reusable_config(&self) -> resources::Reusable_config<'_> {
        resources::Reusable_config::new(self.provider)
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
