//! Lifesciences_api Service
//!
//! Auto-generated service module for lifesciences_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lifesciences_api
pub struct Lifesciences_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Lifesciences_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
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
