//! Spanner_api Service
//!
//! Auto-generated service module for spanner_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for spanner_api
pub struct Spanner_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Spanner_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get scan resource handler
    pub fn scan(&self) -> resources::Scan<'_> {
        resources::Scan::new(self.provider)
    }
    /// Get instance_partition resource handler
    pub fn instance_partition(&self) -> resources::Instance_partition<'_> {
        resources::Instance_partition::new(self.provider)
    }
    /// Get instance_partition_operation resource handler
    pub fn instance_partition_operation(&self) -> resources::Instance_partition_operation<'_> {
        resources::Instance_partition_operation::new(self.provider)
    }
    /// Get backup_operation resource handler
    pub fn backup_operation(&self) -> resources::Backup_operation<'_> {
        resources::Backup_operation::new(self.provider)
    }
    /// Get database_role resource handler
    pub fn database_role(&self) -> resources::Database_role<'_> {
        resources::Database_role::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get database_operation resource handler
    pub fn database_operation(&self) -> resources::Database_operation<'_> {
        resources::Database_operation::new(self.provider)
    }
    /// Get instance_config_operation resource handler
    pub fn instance_config_operation(&self) -> resources::Instance_config_operation<'_> {
        resources::Instance_config_operation::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get instance_config resource handler
    pub fn instance_config(&self) -> resources::Instance_config<'_> {
        resources::Instance_config::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get backup_schedule resource handler
    pub fn backup_schedule(&self) -> resources::Backup_schedule<'_> {
        resources::Backup_schedule::new(self.provider)
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
