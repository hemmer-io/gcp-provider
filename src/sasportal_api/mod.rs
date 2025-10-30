//! Sasportal_api Service
//!
//! Auto-generated service module for sasportal_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sasportal_api
pub struct Sasportal_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sasportal_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get installer resource handler
    pub fn installer(&self) -> resources::Installer<'_> {
        resources::Installer::new(self.provider)
    }
    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
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
