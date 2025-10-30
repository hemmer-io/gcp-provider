//! Eventarc_api Service
//!
//! Auto-generated service module for eventarc_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for eventarc_api
pub struct Eventarc_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Eventarc_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get trigger resource handler
    pub fn trigger(&self) -> resources::Trigger<'_> {
        resources::Trigger::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
