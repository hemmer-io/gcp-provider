//! Doubleclickbidmanager_api Service
//!
//! Auto-generated service module for doubleclickbidmanager_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for doubleclickbidmanager_api
pub struct Doubleclickbidmanager_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Doubleclickbidmanager_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get querie resource handler
    pub fn querie(&self) -> resources::Querie<'_> {
        resources::Querie::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
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
