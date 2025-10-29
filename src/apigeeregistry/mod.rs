//! Apigeeregistry Service
//!
//! Auto-generated service module for apigeeregistry

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigeeregistry
pub struct ApigeeregistryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ApigeeregistryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get artifact resource handler
    pub fn artifact(&self) -> resources::Artifact<'_> {
        resources::Artifact::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get runtime resource handler
    pub fn runtime(&self) -> resources::Runtime<'_> {
        resources::Runtime::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get spec resource handler
    pub fn spec(&self) -> resources::Spec<'_> {
        resources::Spec::new(self.provider)
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
