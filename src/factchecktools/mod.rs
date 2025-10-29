//! Factchecktools Service
//!
//! Auto-generated service module for factchecktools

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for factchecktools
pub struct FactchecktoolsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FactchecktoolsService<'a> {
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
