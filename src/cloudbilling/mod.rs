//! Cloudbilling Service
//!
//! Auto-generated service module for cloudbilling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudbilling
pub struct CloudbillingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudbillingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get sku_group resource handler
    pub fn sku_group(&self) -> resources::Sku_group<'_> {
        resources::Sku_group::new(self.provider)
    }
    /// Get sku resource handler
    pub fn sku(&self) -> resources::Sku<'_> {
        resources::Sku::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get price resource handler
    pub fn price(&self) -> resources::Price<'_> {
        resources::Price::new(self.provider)
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
