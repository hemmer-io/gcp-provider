//! Oracledatabase_api Service
//!
//! Auto-generated service module for oracledatabase_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for oracledatabase_api
pub struct Oracledatabase_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Oracledatabase_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get gi_version resource handler
    pub fn gi_version(&self) -> resources::Gi_version<'_> {
        resources::Gi_version::new(self.provider)
    }
    /// Get cloud_exadata_infrastructure resource handler
    pub fn cloud_exadata_infrastructure(&self) -> resources::Cloud_exadata_infrastructure<'_> {
        resources::Cloud_exadata_infrastructure::new(self.provider)
    }
    /// Get db_system_shape resource handler
    pub fn db_system_shape(&self) -> resources::Db_system_shape<'_> {
        resources::Db_system_shape::new(self.provider)
    }
    /// Get db_server resource handler
    pub fn db_server(&self) -> resources::Db_server<'_> {
        resources::Db_server::new(self.provider)
    }
    /// Get db_system_initial_storage_size resource handler
    pub fn db_system_initial_storage_size(&self) -> resources::Db_system_initial_storage_size<'_> {
        resources::Db_system_initial_storage_size::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get db_node resource handler
    pub fn db_node(&self) -> resources::Db_node<'_> {
        resources::Db_node::new(self.provider)
    }
    /// Get db_version resource handler
    pub fn db_version(&self) -> resources::Db_version<'_> {
        resources::Db_version::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get exascale_db_storage_vault resource handler
    pub fn exascale_db_storage_vault(&self) -> resources::Exascale_db_storage_vault<'_> {
        resources::Exascale_db_storage_vault::new(self.provider)
    }
    /// Get odb_subnet resource handler
    pub fn odb_subnet(&self) -> resources::Odb_subnet<'_> {
        resources::Odb_subnet::new(self.provider)
    }
    /// Get pluggable_database resource handler
    pub fn pluggable_database(&self) -> resources::Pluggable_database<'_> {
        resources::Pluggable_database::new(self.provider)
    }
    /// Get autonomous_db_version resource handler
    pub fn autonomous_db_version(&self) -> resources::Autonomous_db_version<'_> {
        resources::Autonomous_db_version::new(self.provider)
    }
    /// Get autonomous_database_character_set resource handler
    pub fn autonomous_database_character_set(&self) -> resources::Autonomous_database_character_set<'_> {
        resources::Autonomous_database_character_set::new(self.provider)
    }
    /// Get odb_network resource handler
    pub fn odb_network(&self) -> resources::Odb_network<'_> {
        resources::Odb_network::new(self.provider)
    }
    /// Get autonomous_database_backup resource handler
    pub fn autonomous_database_backup(&self) -> resources::Autonomous_database_backup<'_> {
        resources::Autonomous_database_backup::new(self.provider)
    }
    /// Get db_system resource handler
    pub fn db_system(&self) -> resources::Db_system<'_> {
        resources::Db_system::new(self.provider)
    }
    /// Get cloud_vm_cluster resource handler
    pub fn cloud_vm_cluster(&self) -> resources::Cloud_vm_cluster<'_> {
        resources::Cloud_vm_cluster::new(self.provider)
    }
    /// Get autonomous_database resource handler
    pub fn autonomous_database(&self) -> resources::Autonomous_database<'_> {
        resources::Autonomous_database::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
    }
    /// Get exadb_vm_cluster resource handler
    pub fn exadb_vm_cluster(&self) -> resources::Exadb_vm_cluster<'_> {
        resources::Exadb_vm_cluster::new(self.provider)
    }
    /// Get database_character_set resource handler
    pub fn database_character_set(&self) -> resources::Database_character_set<'_> {
        resources::Database_character_set::new(self.provider)
    }
    /// Get minor_version resource handler
    pub fn minor_version(&self) -> resources::Minor_version<'_> {
        resources::Minor_version::new(self.provider)
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
