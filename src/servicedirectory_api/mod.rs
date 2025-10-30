//! Servicedirectory_api Service
//!
//! Auto-generated service module for servicedirectory_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for servicedirectory_api
pub struct Servicedirectory_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Servicedirectory_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
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
