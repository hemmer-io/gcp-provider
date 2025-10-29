//! Parametermanager Service
//!
//! Auto-generated service module for parametermanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for parametermanager
pub struct ParametermanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ParametermanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get parameter resource handler
    pub fn parameter(&self) -> resources::Parameter<'_> {
        resources::Parameter::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
