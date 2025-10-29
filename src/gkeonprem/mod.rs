//! Gkeonprem Service
//!
//! Auto-generated service module for gkeonprem

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gkeonprem
pub struct GkeonpremService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GkeonpremService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get bare_metal_admin_cluster resource handler
    pub fn bare_metal_admin_cluster(&self) -> resources::Bare_metal_admin_cluster<'_> {
        resources::Bare_metal_admin_cluster::new(self.provider)
    }
    /// Get vmware_admin_cluster resource handler
    pub fn vmware_admin_cluster(&self) -> resources::Vmware_admin_cluster<'_> {
        resources::Vmware_admin_cluster::new(self.provider)
    }
    /// Get vmware_node_pool resource handler
    pub fn vmware_node_pool(&self) -> resources::Vmware_node_pool<'_> {
        resources::Vmware_node_pool::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get bare_metal_cluster resource handler
    pub fn bare_metal_cluster(&self) -> resources::Bare_metal_cluster<'_> {
        resources::Bare_metal_cluster::new(self.provider)
    }
    /// Get bare_metal_node_pool resource handler
    pub fn bare_metal_node_pool(&self) -> resources::Bare_metal_node_pool<'_> {
        resources::Bare_metal_node_pool::new(self.provider)
    }
    /// Get vmware_cluster resource handler
    pub fn vmware_cluster(&self) -> resources::Vmware_cluster<'_> {
        resources::Vmware_cluster::new(self.provider)
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
