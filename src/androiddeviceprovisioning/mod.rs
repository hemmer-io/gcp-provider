//! Androiddeviceprovisioning Service
//!
//! Auto-generated service module for androiddeviceprovisioning

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for androiddeviceprovisioning
pub struct AndroiddeviceprovisioningService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AndroiddeviceprovisioningService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get dpc resource handler
    pub fn dpc(&self) -> resources::Dpc<'_> {
        resources::Dpc::new(self.provider)
    }
    /// Get vendor resource handler
    pub fn vendor(&self) -> resources::Vendor<'_> {
        resources::Vendor::new(self.provider)
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
