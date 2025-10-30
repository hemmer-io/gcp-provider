//! Osconfig_api Service
//!
//! Auto-generated service module for osconfig_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for osconfig_api
pub struct Osconfig_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Osconfig_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get instance_detail resource handler
    pub fn instance_detail(&self) -> resources::Instance_detail<'_> {
        resources::Instance_detail::new(self.provider)
    }
    /// Get guest_policie resource handler
    pub fn guest_policie(&self) -> resources::Guest_policie<'_> {
        resources::Guest_policie::new(self.provider)
    }
    /// Get patch_deployment resource handler
    pub fn patch_deployment(&self) -> resources::Patch_deployment<'_> {
        resources::Patch_deployment::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get patch_job resource handler
    pub fn patch_job(&self) -> resources::Patch_job<'_> {
        resources::Patch_job::new(self.provider)
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
