//! Jobs_api Service
//!
//! Auto-generated service module for jobs_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for jobs_api
pub struct Jobs_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Jobs_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get client_event resource handler
    pub fn client_event(&self) -> resources::Client_event<'_> {
        resources::Client_event::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get companie resource handler
    pub fn companie(&self) -> resources::Companie<'_> {
        resources::Companie::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
