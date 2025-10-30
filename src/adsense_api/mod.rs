//! Adsense_api Service
//!
//! Auto-generated service module for adsense_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adsense_api
pub struct Adsense_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adsense_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get adunit resource handler
    pub fn adunit(&self) -> resources::Adunit<'_> {
        resources::Adunit::new(self.provider)
    }
    /// Get adclient resource handler
    pub fn adclient(&self) -> resources::Adclient<'_> {
        resources::Adclient::new(self.provider)
    }
    /// Get dimension resource handler
    pub fn dimension(&self) -> resources::Dimension<'_> {
        resources::Dimension::new(self.provider)
    }
    /// Get metric resource handler
    pub fn metric(&self) -> resources::Metric<'_> {
        resources::Metric::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get savedadstyle resource handler
    pub fn savedadstyle(&self) -> resources::Savedadstyle<'_> {
        resources::Savedadstyle::new(self.provider)
    }
    /// Get urlchannel resource handler
    pub fn urlchannel(&self) -> resources::Urlchannel<'_> {
        resources::Urlchannel::new(self.provider)
    }
    /// Get alert resource handler
    pub fn alert(&self) -> resources::Alert<'_> {
        resources::Alert::new(self.provider)
    }
    /// Get customchannel resource handler
    pub fn customchannel(&self) -> resources::Customchannel<'_> {
        resources::Customchannel::new(self.provider)
    }
    /// Get saved resource handler
    pub fn saved(&self) -> resources::Saved<'_> {
        resources::Saved::new(self.provider)
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
