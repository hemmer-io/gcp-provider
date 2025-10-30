//! Domainsrdap_api Service
//!
//! Auto-generated service module for domainsrdap_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for domainsrdap_api
pub struct Domainsrdap_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Domainsrdap_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get nameserver resource handler
    pub fn nameserver(&self) -> resources::Nameserver<'_> {
        resources::Nameserver::new(self.provider)
    }
    /// Get autnum resource handler
    pub fn autnum(&self) -> resources::Autnum<'_> {
        resources::Autnum::new(self.provider)
    }
    /// Get domainsrdap resource handler
    pub fn domainsrdap(&self) -> resources::Domainsrdap<'_> {
        resources::Domainsrdap::new(self.provider)
    }
    /// Get entity resource handler
    pub fn entity(&self) -> resources::Entity<'_> {
        resources::Entity::new(self.provider)
    }
    /// Get ip resource handler
    pub fn ip(&self) -> resources::Ip<'_> {
        resources::Ip::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
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
