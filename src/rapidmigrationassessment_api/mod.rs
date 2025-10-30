//! Rapidmigrationassessment_api Service
//!
//! Auto-generated service module for rapidmigrationassessment_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rapidmigrationassessment_api
pub struct Rapidmigrationassessment_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Rapidmigrationassessment_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get annotation resource handler
    pub fn annotation(&self) -> resources::Annotation<'_> {
        resources::Annotation::new(self.provider)
    }
    /// Get collector resource handler
    pub fn collector(&self) -> resources::Collector<'_> {
        resources::Collector::new(self.provider)
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
