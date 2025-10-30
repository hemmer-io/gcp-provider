//! Iamcredentials_api Service
//!
//! Auto-generated service module for iamcredentials_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iamcredentials_api
pub struct Iamcredentials_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Iamcredentials_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get service_account resource handler
    pub fn service_account(&self) -> resources::Service_account<'_> {
        resources::Service_account::new(self.provider)
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
