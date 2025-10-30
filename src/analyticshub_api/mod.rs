//! Analyticshub_api Service
//!
//! Auto-generated service module for analyticshub_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analyticshub_api
pub struct Analyticshub_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Analyticshub_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get data_exchange resource handler
    pub fn data_exchange(&self) -> resources::Data_exchange<'_> {
        resources::Data_exchange::new(self.provider)
    }
    /// Get listing resource handler
    pub fn listing(&self) -> resources::Listing<'_> {
        resources::Listing::new(self.provider)
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
