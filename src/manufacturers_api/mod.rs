//! Manufacturers_api Service
//!
//! Auto-generated service module for manufacturers_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for manufacturers_api
pub struct Manufacturers_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Manufacturers_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get product_certification resource handler
    pub fn product_certification(&self) -> resources::Product_certification<'_> {
        resources::Product_certification::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
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
