//! Cloudprivatecatalog Service
//!
//! Auto-generated service module for cloudprivatecatalog

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudprivatecatalog
pub struct CloudprivatecatalogService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudprivatecatalogService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
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
