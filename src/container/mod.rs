//! Container Service
//!
//! Auto-generated service module for container

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for container
pub struct ContainerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ContainerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get node_pool resource handler
    pub fn node_pool(&self) -> resources::Node_pool<'_> {
        resources::Node_pool::new(self.provider)
    }
    /// Get well-known resource handler
    pub fn well-known(&self) -> resources::Well-known<'_> {
        resources::Well-known::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
    }
    /// Get usable_subnetwork resource handler
    pub fn usable_subnetwork(&self) -> resources::Usable_subnetwork<'_> {
        resources::Usable_subnetwork::new(self.provider)
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
