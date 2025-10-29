//! Rapidmigrationassessment Service
//!
//! Auto-generated service module for rapidmigrationassessment

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rapidmigrationassessment
pub struct RapidmigrationassessmentService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> RapidmigrationassessmentService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get annotation resource handler
    pub fn annotation(&self) -> resources::Annotation<'_> {
        resources::Annotation::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get collector resource handler
    pub fn collector(&self) -> resources::Collector<'_> {
        resources::Collector::new(self.provider)
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
