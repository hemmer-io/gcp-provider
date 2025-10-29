//! Composer Service
//!
//! Auto-generated service module for composer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for composer
pub struct ComposerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ComposerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get image_version resource handler
    pub fn image_version(&self) -> resources::Image_version<'_> {
        resources::Image_version::new(self.provider)
    }
    /// Get user_workloads_config_map resource handler
    pub fn user_workloads_config_map(&self) -> resources::User_workloads_config_map<'_> {
        resources::User_workloads_config_map::new(self.provider)
    }
    /// Get user_workloads_secret resource handler
    pub fn user_workloads_secret(&self) -> resources::User_workloads_secret<'_> {
        resources::User_workloads_secret::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
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
