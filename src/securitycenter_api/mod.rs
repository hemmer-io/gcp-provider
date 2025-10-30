//! Securitycenter_api Service
//!
//! Auto-generated service module for securitycenter_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for securitycenter_api
pub struct Securitycenter_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Securitycenter_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get finding resource handler
    pub fn finding(&self) -> resources::Finding<'_> {
        resources::Finding::new(self.provider)
    }
    /// Get source resource handler
    pub fn source(&self) -> resources::Source<'_> {
        resources::Source::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get notification_config resource handler
    pub fn notification_config(&self) -> resources::Notification_config<'_> {
        resources::Notification_config::new(self.provider)
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
