//! Gkehub_api Service
//!
//! Auto-generated service module for gkehub_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gkehub_api
pub struct Gkehub_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Gkehub_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get feature resource handler
    pub fn feature(&self) -> resources::Feature<'_> {
        resources::Feature::new(self.provider)
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
