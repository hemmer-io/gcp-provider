//! Dataportability_api Service
//!
//! Auto-generated service module for dataportability_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataportability_api
pub struct Dataportability_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Dataportability_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get archive_job resource handler
    pub fn archive_job(&self) -> resources::Archive_job<'_> {
        resources::Archive_job::new(self.provider)
    }
    /// Get authorization resource handler
    pub fn authorization(&self) -> resources::Authorization<'_> {
        resources::Authorization::new(self.provider)
    }
    /// Get portability_archive resource handler
    pub fn portability_archive(&self) -> resources::Portability_archive<'_> {
        resources::Portability_archive::new(self.provider)
    }
    /// Get access_type resource handler
    pub fn access_type(&self) -> resources::Access_type<'_> {
        resources::Access_type::new(self.provider)
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
