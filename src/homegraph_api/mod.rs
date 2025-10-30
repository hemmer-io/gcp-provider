//! Homegraph_api Service
//!
//! Auto-generated service module for homegraph_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for homegraph_api
pub struct Homegraph_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Homegraph_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get agent_user resource handler
    pub fn agent_user(&self) -> resources::Agent_user<'_> {
        resources::Agent_user::new(self.provider)
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
