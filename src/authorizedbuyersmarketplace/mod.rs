//! Authorizedbuyersmarketplace Service
//!
//! Auto-generated service module for authorizedbuyersmarketplace

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for authorizedbuyersmarketplace
pub struct AuthorizedbuyersmarketplaceService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AuthorizedbuyersmarketplaceService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get data_segment resource handler
    pub fn data_segment(&self) -> resources::Data_segment<'_> {
        resources::Data_segment::new(self.provider)
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
