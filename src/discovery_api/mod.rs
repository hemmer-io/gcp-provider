//! Discovery_api Service
//!
//! Auto-generated service module for discovery_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for discovery_api
pub struct Discovery_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Discovery_apiService<'a> {
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
