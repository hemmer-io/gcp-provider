//! Batch_api Service
//!
//! Auto-generated service module for batch_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for batch_api
pub struct Batch_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Batch_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get state resource handler
    pub fn state(&self) -> resources::State<'_> {
        resources::State::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
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
