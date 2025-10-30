//! Storagebatchoperations_api Service
//!
//! Auto-generated service module for storagebatchoperations_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for storagebatchoperations_api
pub struct Storagebatchoperations_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Storagebatchoperations_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
