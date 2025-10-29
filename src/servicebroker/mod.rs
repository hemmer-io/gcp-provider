//! Servicebroker Service
//!
//! Auto-generated service module for servicebroker

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for servicebroker
pub struct ServicebrokerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServicebrokerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get service_instance resource handler
    pub fn service_instance(&self) -> resources::Service_instance<'_> {
        resources::Service_instance::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get service_binding resource handler
    pub fn service_binding(&self) -> resources::Service_binding<'_> {
        resources::Service_binding::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
    }
    /// Get servicebroker resource handler
    pub fn servicebroker(&self) -> resources::Servicebroker<'_> {
        resources::Servicebroker::new(self.provider)
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
