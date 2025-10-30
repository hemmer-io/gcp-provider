//! Workflows_api Service
//!
//! Auto-generated service module for workflows_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workflows_api
pub struct Workflows_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Workflows_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get workflow resource handler
    pub fn workflow(&self) -> resources::Workflow<'_> {
        resources::Workflow::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
