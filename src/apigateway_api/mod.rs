//! Apigateway_api Service
//!
//! Auto-generated service module for apigateway_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigateway_api
pub struct Apigateway_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apigateway_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get config resource handler
    pub fn config(&self) -> resources::Config<'_> {
        resources::Config::new(self.provider)
    }
    /// Get gateway resource handler
    pub fn gateway(&self) -> resources::Gateway<'_> {
        resources::Gateway::new(self.provider)
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
