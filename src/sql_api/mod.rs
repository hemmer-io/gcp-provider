//! Sql_api Service
//!
//! Auto-generated service module for sql_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sql_api
pub struct Sql_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sql_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get tier resource handler
    pub fn tier(&self) -> resources::Tier<'_> {
        resources::Tier::new(self.provider)
    }
    /// Get flag resource handler
    pub fn flag(&self) -> resources::Flag<'_> {
        resources::Flag::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get ssl_cert resource handler
    pub fn ssl_cert(&self) -> resources::Ssl_cert<'_> {
        resources::Ssl_cert::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get backup_run resource handler
    pub fn backup_run(&self) -> resources::Backup_run<'_> {
        resources::Backup_run::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
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
