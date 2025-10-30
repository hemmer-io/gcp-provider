//! Datamigration_api Service
//!
//! Auto-generated service module for datamigration_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datamigration_api
pub struct Datamigration_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Datamigration_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get connection_profile resource handler
    pub fn connection_profile(&self) -> resources::Connection_profile<'_> {
        resources::Connection_profile::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get migration_job resource handler
    pub fn migration_job(&self) -> resources::Migration_job<'_> {
        resources::Migration_job::new(self.provider)
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
