//! Cloudcommerceprocurement Service
//!
//! Auto-generated service module for cloudcommerceprocurement

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudcommerceprocurement
pub struct CloudcommerceprocurementService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudcommerceprocurementService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
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
