//! Smartdevicemanagement_api Service
//!
//! Auto-generated service module for smartdevicemanagement_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for smartdevicemanagement_api
pub struct Smartdevicemanagement_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Smartdevicemanagement_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get structure resource handler
    pub fn structure(&self) -> resources::Structure<'_> {
        resources::Structure::new(self.provider)
    }
    /// Get room resource handler
    pub fn room(&self) -> resources::Room<'_> {
        resources::Room::new(self.provider)
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
