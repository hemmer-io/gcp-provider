//! Looker_api Service
//!
//! Auto-generated service module for looker_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for looker_api
pub struct Looker_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Looker_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
