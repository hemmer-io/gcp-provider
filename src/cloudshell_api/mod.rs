//! Cloudshell_api Service
//!
//! Auto-generated service module for cloudshell_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudshell_api
pub struct Cloudshell_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudshell_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get public_key resource handler
    pub fn public_key(&self) -> resources::Public_key<'_> {
        resources::Public_key::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
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
