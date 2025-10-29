//! Kmsinventory Service
//!
//! Auto-generated service module for kmsinventory

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kmsinventory
pub struct KmsinventoryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> KmsinventoryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get crypto_key resource handler
    pub fn crypto_key(&self) -> resources::Crypto_key<'_> {
        resources::Crypto_key::new(self.provider)
    }
    /// Get protected_resource resource handler
    pub fn protected_resource(&self) -> resources::Protected_resource<'_> {
        resources::Protected_resource::new(self.provider)
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
