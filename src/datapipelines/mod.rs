//! Datapipelines Service
//!
//! Auto-generated service module for datapipelines

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datapipelines
pub struct DatapipelinesService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DatapipelinesService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
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
