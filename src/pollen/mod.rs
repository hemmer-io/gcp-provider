//! Pollen Service
//!
//! Auto-generated service module for pollen

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pollen
pub struct PollenService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PollenService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
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
