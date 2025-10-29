//! Homegraph Service
//!
//! Auto-generated service module for homegraph

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for homegraph
pub struct HomegraphService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> HomegraphService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get agent_user resource handler
    pub fn agent_user(&self) -> resources::Agent_user<'_> {
        resources::Agent_user::new(self.provider)
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
