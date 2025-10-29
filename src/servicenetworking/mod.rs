//! Servicenetworking Service
//!
//! Auto-generated service module for servicenetworking

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for servicenetworking
pub struct ServicenetworkingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServicenetworkingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
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
