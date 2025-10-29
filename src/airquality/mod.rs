//! Airquality Service
//!
//! Auto-generated service module for airquality

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for airquality
pub struct AirqualityService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AirqualityService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get current_condition resource handler
    pub fn current_condition(&self) -> resources::Current_condition<'_> {
        resources::Current_condition::new(self.provider)
    }
    /// Get history resource handler
    pub fn history(&self) -> resources::History<'_> {
        resources::History::new(self.provider)
    }
    /// Get heatmap_tile resource handler
    pub fn heatmap_tile(&self) -> resources::Heatmap_tile<'_> {
        resources::Heatmap_tile::new(self.provider)
    }
    /// Get forecast resource handler
    pub fn forecast(&self) -> resources::Forecast<'_> {
        resources::Forecast::new(self.provider)
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
