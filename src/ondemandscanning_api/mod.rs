//! Ondemandscanning_api Service
//!
//! Auto-generated service module for ondemandscanning_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ondemandscanning_api
pub struct Ondemandscanning_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Ondemandscanning_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get vulnerabilitie resource handler
    pub fn vulnerabilitie(&self) -> resources::Vulnerabilitie<'_> {
        resources::Vulnerabilitie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get scan resource handler
    pub fn scan(&self) -> resources::Scan<'_> {
        resources::Scan::new(self.provider)
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
