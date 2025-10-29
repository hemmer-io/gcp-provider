//! Trafficdirector Service
//!
//! Auto-generated service module for trafficdirector

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for trafficdirector
pub struct TrafficdirectorService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TrafficdirectorService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get discovery resource handler
    pub fn discovery(&self) -> resources::Discovery<'_> {
        resources::Discovery::new(self.provider)
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
