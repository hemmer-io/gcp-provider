//! Ml_api Service
//!
//! Auto-generated service module for ml_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ml_api
pub struct Ml_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Ml_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get studie resource handler
    pub fn studie(&self) -> resources::Studie<'_> {
        resources::Studie::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get trial resource handler
    pub fn trial(&self) -> resources::Trial<'_> {
        resources::Trial::new(self.provider)
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
