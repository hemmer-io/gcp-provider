//! Metastore Service
//!
//! Auto-generated service module for metastore

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for metastore
pub struct MetastoreService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MetastoreService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get migration_execution resource handler
    pub fn migration_execution(&self) -> resources::Migration_execution<'_> {
        resources::Migration_execution::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
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
