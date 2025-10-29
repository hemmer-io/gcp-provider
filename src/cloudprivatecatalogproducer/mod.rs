//! Cloudprivatecatalogproducer Service
//!
//! Auto-generated service module for cloudprivatecatalogproducer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudprivatecatalogproducer
pub struct CloudprivatecatalogproducerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudprivatecatalogproducerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get icon resource handler
    pub fn icon(&self) -> resources::Icon<'_> {
        resources::Icon::new(self.provider)
    }
    /// Get association resource handler
    pub fn association(&self) -> resources::Association<'_> {
        resources::Association::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
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
