//! Accesscontextmanager Service
//!
//! Auto-generated service module for accesscontextmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for accesscontextmanager
pub struct AccesscontextmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AccesscontextmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get access_level resource handler
    pub fn access_level(&self) -> resources::Access_level<'_> {
        resources::Access_level::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get access_policie resource handler
    pub fn access_policie(&self) -> resources::Access_policie<'_> {
        resources::Access_policie::new(self.provider)
    }
    /// Get service_perimeter resource handler
    pub fn service_perimeter(&self) -> resources::Service_perimeter<'_> {
        resources::Service_perimeter::new(self.provider)
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
