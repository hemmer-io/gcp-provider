//! Config Service
//!
//! Auto-generated service module for config

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for config
pub struct ConfigService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ConfigService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get preview resource handler
    pub fn preview(&self) -> resources::Preview<'_> {
        resources::Preview::new(self.provider)
    }
    /// Get terraform_version resource handler
    pub fn terraform_version(&self) -> resources::Terraform_version<'_> {
        resources::Terraform_version::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get resource_change resource handler
    pub fn resource_change(&self) -> resources::Resource_change<'_> {
        resources::Resource_change::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get resource_drift resource handler
    pub fn resource_drift(&self) -> resources::Resource_drift<'_> {
        resources::Resource_drift::new(self.provider)
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
