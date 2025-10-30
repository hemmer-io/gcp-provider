//! Chromepolicy_api Service
//!
//! Auto-generated service module for chromepolicy_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chromepolicy_api
pub struct Chromepolicy_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Chromepolicy_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get network resource handler
    pub fn network(&self) -> resources::Network<'_> {
        resources::Network::new(self.provider)
    }
    /// Get policy_schema resource handler
    pub fn policy_schema(&self) -> resources::Policy_schema<'_> {
        resources::Policy_schema::new(self.provider)
    }
    /// Get orgunit resource handler
    pub fn orgunit(&self) -> resources::Orgunit<'_> {
        resources::Orgunit::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
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
