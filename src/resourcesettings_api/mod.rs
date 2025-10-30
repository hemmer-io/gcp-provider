//! Resourcesettings_api Service
//!
//! Auto-generated service module for resourcesettings_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resourcesettings_api
pub struct Resourcesettings_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Resourcesettings_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get setting resource handler
    pub fn setting(&self) -> resources::Setting<'_> {
        resources::Setting::new(self.provider)
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
