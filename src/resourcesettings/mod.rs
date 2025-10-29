//! Resourcesettings Service
//!
//! Auto-generated service module for resourcesettings

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resourcesettings
pub struct ResourcesettingsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ResourcesettingsService<'a> {
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
