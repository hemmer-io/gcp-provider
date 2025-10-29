//! Testing Service
//!
//! Auto-generated service module for testing

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for testing
pub struct TestingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TestingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get application_detail_service resource handler
    pub fn application_detail_service(&self) -> resources::Application_detail_service<'_> {
        resources::Application_detail_service::new(self.provider)
    }
    /// Get test_matrice resource handler
    pub fn test_matrice(&self) -> resources::Test_matrice<'_> {
        resources::Test_matrice::new(self.provider)
    }
    /// Get test_environment_catalog resource handler
    pub fn test_environment_catalog(&self) -> resources::Test_environment_catalog<'_> {
        resources::Test_environment_catalog::new(self.provider)
    }
    /// Get device_session resource handler
    pub fn device_session(&self) -> resources::Device_session<'_> {
        resources::Device_session::new(self.provider)
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
