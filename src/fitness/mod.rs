//! Fitness Service
//!
//! Auto-generated service module for fitness

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fitness
pub struct FitnessService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FitnessService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get data_point_change resource handler
    pub fn data_point_change(&self) -> resources::Data_point_change<'_> {
        resources::Data_point_change::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
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
