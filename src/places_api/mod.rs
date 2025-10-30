//! Places_api Service
//!
//! Auto-generated service module for places_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for places_api
pub struct Places_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Places_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get photo resource handler
    pub fn photo(&self) -> resources::Photo<'_> {
        resources::Photo::new(self.provider)
    }
    /// Get place resource handler
    pub fn place(&self) -> resources::Place<'_> {
        resources::Place::new(self.provider)
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
