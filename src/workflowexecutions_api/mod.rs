//! Workflowexecutions_api Service
//!
//! Auto-generated service module for workflowexecutions_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workflowexecutions_api
pub struct Workflowexecutions_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Workflowexecutions_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
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
