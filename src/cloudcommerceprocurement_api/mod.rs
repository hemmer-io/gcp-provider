//! Cloudcommerceprocurement_api Service
//!
//! Auto-generated service module for cloudcommerceprocurement_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudcommerceprocurement_api
pub struct Cloudcommerceprocurement_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudcommerceprocurement_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
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
