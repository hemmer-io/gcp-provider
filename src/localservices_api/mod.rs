//! Localservices_api Service
//!
//! Auto-generated service module for localservices_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for localservices_api
pub struct Localservices_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Localservices_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get account_report resource handler
    pub fn account_report(&self) -> resources::Account_report<'_> {
        resources::Account_report::new(self.provider)
    }
    /// Get detailed_lead_report resource handler
    pub fn detailed_lead_report(&self) -> resources::Detailed_lead_report<'_> {
        resources::Detailed_lead_report::new(self.provider)
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
