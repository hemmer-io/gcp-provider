//! Run_api Service
//!
//! Auto-generated service module for run_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for run_api
pub struct Run_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Run_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
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
