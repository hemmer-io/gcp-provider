//! Datafusion_api Service
//!
//! Auto-generated service module for datafusion_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datafusion_api
pub struct Datafusion_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Datafusion_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
    }
    /// Get dns_peering resource handler
    pub fn dns_peering(&self) -> resources::Dns_peering<'_> {
        resources::Dns_peering::new(self.provider)
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
