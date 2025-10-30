//! Parametermanager_api Service
//!
//! Auto-generated service module for parametermanager_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for parametermanager_api
pub struct Parametermanager_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Parametermanager_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get parameter resource handler
    pub fn parameter(&self) -> resources::Parameter<'_> {
        resources::Parameter::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
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
