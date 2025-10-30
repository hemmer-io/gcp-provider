//! Factchecktools_api Service
//!
//! Auto-generated service module for factchecktools_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for factchecktools_api
pub struct Factchecktools_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Factchecktools_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get claim resource handler
    pub fn claim(&self) -> resources::Claim<'_> {
        resources::Claim::new(self.provider)
    }
    /// Get page resource handler
    pub fn page(&self) -> resources::Page<'_> {
        resources::Page::new(self.provider)
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
