//! Advisorynotifications_api Service
//!
//! Auto-generated service module for advisorynotifications_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for advisorynotifications_api
pub struct Advisorynotifications_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Advisorynotifications_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get notification resource handler
    pub fn notification(&self) -> resources::Notification<'_> {
        resources::Notification::new(self.provider)
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
