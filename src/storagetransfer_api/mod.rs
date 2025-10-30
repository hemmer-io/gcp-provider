//! Storagetransfer_api Service
//!
//! Auto-generated service module for storagetransfer_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for storagetransfer_api
pub struct Storagetransfer_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Storagetransfer_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get transfer_operation resource handler
    pub fn transfer_operation(&self) -> resources::Transfer_operation<'_> {
        resources::Transfer_operation::new(self.provider)
    }
    /// Get agent_pool resource handler
    pub fn agent_pool(&self) -> resources::Agent_pool<'_> {
        resources::Agent_pool::new(self.provider)
    }
    /// Get google_service_account resource handler
    pub fn google_service_account(&self) -> resources::Google_service_account<'_> {
        resources::Google_service_account::new(self.provider)
    }
    /// Get transfer_job resource handler
    pub fn transfer_job(&self) -> resources::Transfer_job<'_> {
        resources::Transfer_job::new(self.provider)
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
