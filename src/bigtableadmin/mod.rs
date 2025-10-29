//! Bigtableadmin Service
//!
//! Auto-generated service module for bigtableadmin

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigtableadmin
pub struct BigtableadminService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BigtableadminService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get materialized_view resource handler
    pub fn materialized_view(&self) -> resources::Materialized_view<'_> {
        resources::Materialized_view::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get app_profile resource handler
    pub fn app_profile(&self) -> resources::App_profile<'_> {
        resources::App_profile::new(self.provider)
    }
    /// Get authorized_view resource handler
    pub fn authorized_view(&self) -> resources::Authorized_view<'_> {
        resources::Authorized_view::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get hot_tablet resource handler
    pub fn hot_tablet(&self) -> resources::Hot_tablet<'_> {
        resources::Hot_tablet::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get logical_view resource handler
    pub fn logical_view(&self) -> resources::Logical_view<'_> {
        resources::Logical_view::new(self.provider)
    }
    /// Get schema_bundle resource handler
    pub fn schema_bundle(&self) -> resources::Schema_bundle<'_> {
        resources::Schema_bundle::new(self.provider)
    }
    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
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
