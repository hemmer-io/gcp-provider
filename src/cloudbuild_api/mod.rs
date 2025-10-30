//! Cloudbuild_api Service
//!
//! Auto-generated service module for cloudbuild_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudbuild_api
pub struct Cloudbuild_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudbuild_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get worker_pool resource handler
    pub fn worker_pool(&self) -> resources::Worker_pool<'_> {
        resources::Worker_pool::new(self.provider)
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
