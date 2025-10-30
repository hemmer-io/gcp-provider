//! Firebaseremoteconfig_api Service
//!
//! Auto-generated service module for firebaseremoteconfig_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseremoteconfig_api
pub struct Firebaseremoteconfig_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebaseremoteconfig_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
