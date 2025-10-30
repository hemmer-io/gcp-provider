//! Videointelligence_api Service
//!
//! Auto-generated service module for videointelligence_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for videointelligence_api
pub struct Videointelligence_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Videointelligence_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get video resource handler
    pub fn video(&self) -> resources::Video<'_> {
        resources::Video::new(self.provider)
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
