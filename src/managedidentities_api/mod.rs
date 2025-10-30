//! Managedidentities_api Service
//!
//! Auto-generated service module for managedidentities_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for managedidentities_api
pub struct Managedidentities_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Managedidentities_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get peering resource handler
    pub fn peering(&self) -> resources::Peering<'_> {
        resources::Peering::new(self.provider)
    }
    /// Get sql_integration resource handler
    pub fn sql_integration(&self) -> resources::Sql_integration<'_> {
        resources::Sql_integration::new(self.provider)
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
