//! Licensing_api Service
//!
//! Auto-generated service module for licensing_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for licensing_api
pub struct Licensing_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Licensing_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get license_assignment resource handler
    pub fn license_assignment(&self) -> resources::License_assignment<'_> {
        resources::License_assignment::new(self.provider)
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
