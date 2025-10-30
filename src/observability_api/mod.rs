//! Observability_api Service
//!
//! Auto-generated service module for observability_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for observability_api
pub struct Observability_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Observability_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get scope resource handler
    pub fn scope(&self) -> resources::Scope<'_> {
        resources::Scope::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get trace_scope resource handler
    pub fn trace_scope(&self) -> resources::Trace_scope<'_> {
        resources::Trace_scope::new(self.provider)
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
