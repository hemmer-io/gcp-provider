//! Workstations Service
//!
//! Auto-generated service module for workstations

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workstations
pub struct WorkstationsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WorkstationsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get workstation_config resource handler
    pub fn workstation_config(&self) -> resources::Workstation_config<'_> {
        resources::Workstation_config::new(self.provider)
    }
    /// Get workstation_cluster resource handler
    pub fn workstation_cluster(&self) -> resources::Workstation_cluster<'_> {
        resources::Workstation_cluster::new(self.provider)
    }
    /// Get workstation resource handler
    pub fn workstation(&self) -> resources::Workstation<'_> {
        resources::Workstation::new(self.provider)
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
