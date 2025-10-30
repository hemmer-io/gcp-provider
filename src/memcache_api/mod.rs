//! Memcache_api Service
//!
//! Auto-generated service module for memcache_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for memcache_api
pub struct Memcache_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Memcache_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
