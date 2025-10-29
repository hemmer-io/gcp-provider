//! Gkebackup Service
//!
//! Auto-generated service module for gkebackup

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gkebackup
pub struct GkebackupService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GkebackupService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get restore_channel resource handler
    pub fn restore_channel(&self) -> resources::Restore_channel<'_> {
        resources::Restore_channel::new(self.provider)
    }
    /// Get backup_channel resource handler
    pub fn backup_channel(&self) -> resources::Backup_channel<'_> {
        resources::Backup_channel::new(self.provider)
    }
    /// Get restore_plan resource handler
    pub fn restore_plan(&self) -> resources::Restore_plan<'_> {
        resources::Restore_plan::new(self.provider)
    }
    /// Get volume_restore resource handler
    pub fn volume_restore(&self) -> resources::Volume_restore<'_> {
        resources::Volume_restore::new(self.provider)
    }
    /// Get backup_plan resource handler
    pub fn backup_plan(&self) -> resources::Backup_plan<'_> {
        resources::Backup_plan::new(self.provider)
    }
    /// Get volume_backup resource handler
    pub fn volume_backup(&self) -> resources::Volume_backup<'_> {
        resources::Volume_backup::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get restore_plan_binding resource handler
    pub fn restore_plan_binding(&self) -> resources::Restore_plan_binding<'_> {
        resources::Restore_plan_binding::new(self.provider)
    }
    /// Get restore resource handler
    pub fn restore(&self) -> resources::Restore<'_> {
        resources::Restore::new(self.provider)
    }
    /// Get backup_plan_binding resource handler
    pub fn backup_plan_binding(&self) -> resources::Backup_plan_binding<'_> {
        resources::Backup_plan_binding::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
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
