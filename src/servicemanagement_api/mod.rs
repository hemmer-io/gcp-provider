//! Servicemanagement_api Service
//!
//! Auto-generated service module for servicemanagement_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for servicemanagement_api
pub struct Servicemanagement_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Servicemanagement_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get consumer resource handler
    pub fn consumer(&self) -> resources::Consumer<'_> {
        resources::Consumer::new(self.provider)
    }
    /// Get config resource handler
    pub fn config(&self) -> resources::Config<'_> {
        resources::Config::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get rollout resource handler
    pub fn rollout(&self) -> resources::Rollout<'_> {
        resources::Rollout::new(self.provider)
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
