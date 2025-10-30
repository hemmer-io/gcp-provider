//! Analyticsadmin_api Service
//!
//! Auto-generated service module for analyticsadmin_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analyticsadmin_api
pub struct Analyticsadmin_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Analyticsadmin_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get custom_metric resource handler
    pub fn custom_metric(&self) -> resources::Custom_metric<'_> {
        resources::Custom_metric::new(self.provider)
    }
    /// Get google_ads_link resource handler
    pub fn google_ads_link(&self) -> resources::Google_ads_link<'_> {
        resources::Google_ads_link::new(self.provider)
    }
    /// Get custom_dimension resource handler
    pub fn custom_dimension(&self) -> resources::Custom_dimension<'_> {
        resources::Custom_dimension::new(self.provider)
    }
    /// Get account_summarie resource handler
    pub fn account_summarie(&self) -> resources::Account_summarie<'_> {
        resources::Account_summarie::new(self.provider)
    }
    /// Get data_stream resource handler
    pub fn data_stream(&self) -> resources::Data_stream<'_> {
        resources::Data_stream::new(self.provider)
    }
    /// Get measurement_protocol_secret resource handler
    pub fn measurement_protocol_secret(&self) -> resources::Measurement_protocol_secret<'_> {
        resources::Measurement_protocol_secret::new(self.provider)
    }
    /// Get conversion_event resource handler
    pub fn conversion_event(&self) -> resources::Conversion_event<'_> {
        resources::Conversion_event::new(self.provider)
    }
    /// Get firebase_link resource handler
    pub fn firebase_link(&self) -> resources::Firebase_link<'_> {
        resources::Firebase_link::new(self.provider)
    }
    /// Get key_event resource handler
    pub fn key_event(&self) -> resources::Key_event<'_> {
        resources::Key_event::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get propertie resource handler
    pub fn propertie(&self) -> resources::Propertie<'_> {
        resources::Propertie::new(self.provider)
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
