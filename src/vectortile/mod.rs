//! Vectortile Service
//!
//! Auto-generated service module for vectortile

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vectortile
pub struct VectortileService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VectortileService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get featuretile resource handler
    pub fn featuretile(&self) -> resources::Featuretile<'_> {
        resources::Featuretile::new(self.provider)
    }
    /// Get terraintile resource handler
    pub fn terraintile(&self) -> resources::Terraintile<'_> {
        resources::Terraintile::new(self.provider)
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
