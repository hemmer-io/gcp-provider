//! Area120tables_api Service
//!
//! Auto-generated service module for area120tables_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for area120tables_api
pub struct Area120tables_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Area120tables_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get row resource handler
    pub fn row(&self) -> resources::Row<'_> {
        resources::Row::new(self.provider)
    }
    /// Get workspace resource handler
    pub fn workspace(&self) -> resources::Workspace<'_> {
        resources::Workspace::new(self.provider)
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
