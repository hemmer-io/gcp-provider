//! Backupdr Service
//!
//! Auto-generated service module for backupdr

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for backupdr
pub struct BackupdrService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BackupdrService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get backup_plan_association resource handler
    pub fn backup_plan_association(&self) -> resources::Backup_plan_association<'_> {
        resources::Backup_plan_association::new(self.provider)
    }
    /// Get service_config resource handler
    pub fn service_config(&self) -> resources::Service_config<'_> {
        resources::Service_config::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get backup_vault resource handler
    pub fn backup_vault(&self) -> resources::Backup_vault<'_> {
        resources::Backup_vault::new(self.provider)
    }
    /// Get backup_plan resource handler
    pub fn backup_plan(&self) -> resources::Backup_plan<'_> {
        resources::Backup_plan::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get management_server resource handler
    pub fn management_server(&self) -> resources::Management_server<'_> {
        resources::Management_server::new(self.provider)
    }
    /// Get resource_backup_config resource handler
    pub fn resource_backup_config(&self) -> resources::Resource_backup_config<'_> {
        resources::Resource_backup_config::new(self.provider)
    }
    /// Get data_source_reference resource handler
    pub fn data_source_reference(&self) -> resources::Data_source_reference<'_> {
        resources::Data_source_reference::new(self.provider)
    }
    /// Get trial resource handler
    pub fn trial(&self) -> resources::Trial<'_> {
        resources::Trial::new(self.provider)
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
