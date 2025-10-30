//! Adsensehost_api Service
//!
//! Auto-generated service module for adsensehost_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adsensehost_api
pub struct Adsensehost_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adsensehost_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get associationsession resource handler
    pub fn associationsession(&self) -> resources::Associationsession<'_> {
        resources::Associationsession::new(self.provider)
    }
    /// Get adclient resource handler
    pub fn adclient(&self) -> resources::Adclient<'_> {
        resources::Adclient::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get adunit resource handler
    pub fn adunit(&self) -> resources::Adunit<'_> {
        resources::Adunit::new(self.provider)
    }
    /// Get urlchannel resource handler
    pub fn urlchannel(&self) -> resources::Urlchannel<'_> {
        resources::Urlchannel::new(self.provider)
    }
    /// Get customchannel resource handler
    pub fn customchannel(&self) -> resources::Customchannel<'_> {
        resources::Customchannel::new(self.provider)
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
