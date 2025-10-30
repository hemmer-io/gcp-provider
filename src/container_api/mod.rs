//! Container_api Service
//!
//! Auto-generated service module for container_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for container_api
pub struct Container_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Container_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get well_known resource handler
    pub fn well_known(&self) -> resources::Well_known<'_> {
        resources::Well_known::new(self.provider)
    }
    /// Get usable_subnetwork resource handler
    pub fn usable_subnetwork(&self) -> resources::Usable_subnetwork<'_> {
        resources::Usable_subnetwork::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get node_pool resource handler
    pub fn node_pool(&self) -> resources::Node_pool<'_> {
        resources::Node_pool::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
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
