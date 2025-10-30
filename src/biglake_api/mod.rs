//! Biglake_api Service
//!
//! Auto-generated service module for biglake_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for biglake_api
pub struct Biglake_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Biglake_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
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
