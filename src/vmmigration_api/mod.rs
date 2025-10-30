//! Vmmigration_api Service
//!
//! Auto-generated service module for vmmigration_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vmmigration_api
pub struct Vmmigration_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Vmmigration_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get image_import_job resource handler
    pub fn image_import_job(&self) -> resources::Image_import_job<'_> {
        resources::Image_import_job::new(self.provider)
    }
    /// Get source resource handler
    pub fn source(&self) -> resources::Source<'_> {
        resources::Source::new(self.provider)
    }
    /// Get utilization_report resource handler
    pub fn utilization_report(&self) -> resources::Utilization_report<'_> {
        resources::Utilization_report::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get disk_migration_job resource handler
    pub fn disk_migration_job(&self) -> resources::Disk_migration_job<'_> {
        resources::Disk_migration_job::new(self.provider)
    }
    /// Get cutover_job resource handler
    pub fn cutover_job(&self) -> resources::Cutover_job<'_> {
        resources::Cutover_job::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get replication_cycle resource handler
    pub fn replication_cycle(&self) -> resources::Replication_cycle<'_> {
        resources::Replication_cycle::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get target_project resource handler
    pub fn target_project(&self) -> resources::Target_project<'_> {
        resources::Target_project::new(self.provider)
    }
    /// Get migrating_vm resource handler
    pub fn migrating_vm(&self) -> resources::Migrating_vm<'_> {
        resources::Migrating_vm::new(self.provider)
    }
    /// Get image_import resource handler
    pub fn image_import(&self) -> resources::Image_import<'_> {
        resources::Image_import::new(self.provider)
    }
    /// Get datacenter_connector resource handler
    pub fn datacenter_connector(&self) -> resources::Datacenter_connector<'_> {
        resources::Datacenter_connector::new(self.provider)
    }
    /// Get clone_job resource handler
    pub fn clone_job(&self) -> resources::Clone_job<'_> {
        resources::Clone_job::new(self.provider)
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
