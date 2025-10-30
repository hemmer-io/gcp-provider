//! Genomics_api Service
//!
//! Auto-generated service module for genomics_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for genomics_api
pub struct Genomics_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Genomics_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get worker resource handler
    pub fn worker(&self) -> resources::Worker<'_> {
        resources::Worker::new(self.provider)
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
