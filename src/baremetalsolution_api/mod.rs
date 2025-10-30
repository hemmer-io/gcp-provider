//! Baremetalsolution_api Service
//!
//! Auto-generated service module for baremetalsolution_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for baremetalsolution_api
pub struct Baremetalsolution_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Baremetalsolution_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get provisioning_quota resource handler
    pub fn provisioning_quota(&self) -> resources::Provisioning_quota<'_> {
        resources::Provisioning_quota::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
