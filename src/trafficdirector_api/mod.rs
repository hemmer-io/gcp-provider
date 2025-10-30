//! Trafficdirector_api Service
//!
//! Auto-generated service module for trafficdirector_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for trafficdirector_api
pub struct Trafficdirector_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Trafficdirector_apiService<'a> {
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
