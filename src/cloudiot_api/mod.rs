//! Cloudiot_api Service
//!
//! Auto-generated service module for cloudiot_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudiot_api
pub struct Cloudiot_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudiot_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get config_version resource handler
    pub fn config_version(&self) -> resources::Config_version<'_> {
        resources::Config_version::new(self.provider)
    }
    /// Get state resource handler
    pub fn state(&self) -> resources::State<'_> {
        resources::State::new(self.provider)
    }
    /// Get registrie resource handler
    pub fn registrie(&self) -> resources::Registrie<'_> {
        resources::Registrie::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
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
