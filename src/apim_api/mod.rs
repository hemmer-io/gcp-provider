//! Apim_api Service
//!
//! Auto-generated service module for apim_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apim_api
pub struct Apim_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apim_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get api_operation resource handler
    pub fn api_operation(&self) -> resources::Api_operation<'_> {
        resources::Api_operation::new(self.provider)
    }
    /// Get observation_job resource handler
    pub fn observation_job(&self) -> resources::Observation_job<'_> {
        resources::Observation_job::new(self.provider)
    }
    /// Get observation_source resource handler
    pub fn observation_source(&self) -> resources::Observation_source<'_> {
        resources::Observation_source::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get api_observation resource handler
    pub fn api_observation(&self) -> resources::Api_observation<'_> {
        resources::Api_observation::new(self.provider)
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
