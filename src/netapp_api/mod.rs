//! Netapp_api Service
//!
//! Auto-generated service module for netapp_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for netapp_api
pub struct Netapp_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Netapp_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get backup_policie resource handler
    pub fn backup_policie(&self) -> resources::Backup_policie<'_> {
        resources::Backup_policie::new(self.provider)
    }
    /// Get quota_rule resource handler
    pub fn quota_rule(&self) -> resources::Quota_rule<'_> {
        resources::Quota_rule::new(self.provider)
    }
    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get kms_config resource handler
    pub fn kms_config(&self) -> resources::Kms_config<'_> {
        resources::Kms_config::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get active_directorie resource handler
    pub fn active_directorie(&self) -> resources::Active_directorie<'_> {
        resources::Active_directorie::new(self.provider)
    }
    /// Get backup_vault resource handler
    pub fn backup_vault(&self) -> resources::Backup_vault<'_> {
        resources::Backup_vault::new(self.provider)
    }
    /// Get storage_pool resource handler
    pub fn storage_pool(&self) -> resources::Storage_pool<'_> {
        resources::Storage_pool::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get replication resource handler
    pub fn replication(&self) -> resources::Replication<'_> {
        resources::Replication::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
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
