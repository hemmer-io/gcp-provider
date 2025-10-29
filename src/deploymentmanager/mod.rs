//! Deploymentmanager Service
//!
//! Auto-generated service module for deploymentmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for deploymentmanager
pub struct DeploymentmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DeploymentmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get manifest resource handler
    pub fn manifest(&self) -> resources::Manifest<'_> {
        resources::Manifest::new(self.provider)
    }
    /// Get type resource handler
    pub fn type(&self) -> resources::Type<'_> {
        resources::Type::new(self.provider)
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
