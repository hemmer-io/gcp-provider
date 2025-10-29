//! Discovery Service
//!
//! Auto-generated service module for discovery

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for discovery
pub struct DiscoveryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DiscoveryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
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
