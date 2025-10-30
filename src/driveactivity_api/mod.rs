//! Driveactivity_api Service
//!
//! Auto-generated service module for driveactivity_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for driveactivity_api
pub struct Driveactivity_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Driveactivity_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get activity resource handler
    pub fn activity(&self) -> resources::Activity<'_> {
        resources::Activity::new(self.provider)
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
