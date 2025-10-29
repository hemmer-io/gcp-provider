//! Migrationcenter Service
//!
//! Auto-generated service module for migrationcenter

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migrationcenter
pub struct MigrationcenterService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MigrationcenterService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get import_job resource handler
    pub fn import_job(&self) -> resources::Import_job<'_> {
        resources::Import_job::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get preference_set resource handler
    pub fn preference_set(&self) -> resources::Preference_set<'_> {
        resources::Preference_set::new(self.provider)
    }
    /// Get report_config resource handler
    pub fn report_config(&self) -> resources::Report_config<'_> {
        resources::Report_config::new(self.provider)
    }
    /// Get source resource handler
    pub fn source(&self) -> resources::Source<'_> {
        resources::Source::new(self.provider)
    }
    /// Get error_frame resource handler
    pub fn error_frame(&self) -> resources::Error_frame<'_> {
        resources::Error_frame::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get relation resource handler
    pub fn relation(&self) -> resources::Relation<'_> {
        resources::Relation::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get import_data_file resource handler
    pub fn import_data_file(&self) -> resources::Import_data_file<'_> {
        resources::Import_data_file::new(self.provider)
    }
    /// Get assets_export_job resource handler
    pub fn assets_export_job(&self) -> resources::Assets_export_job<'_> {
        resources::Assets_export_job::new(self.provider)
    }
    /// Get discovery_client resource handler
    pub fn discovery_client(&self) -> resources::Discovery_client<'_> {
        resources::Discovery_client::new(self.provider)
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
