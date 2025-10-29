//! Solar Service
//!
//! Auto-generated service module for solar

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for solar
pub struct SolarService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SolarService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get building_insight resource handler
    pub fn building_insight(&self) -> resources::Building_insight<'_> {
        resources::Building_insight::new(self.provider)
    }
    /// Get data_layer resource handler
    pub fn data_layer(&self) -> resources::Data_layer<'_> {
        resources::Data_layer::new(self.provider)
    }
    /// Get geo_tiff resource handler
    pub fn geo_tiff(&self) -> resources::Geo_tiff<'_> {
        resources::Geo_tiff::new(self.provider)
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
