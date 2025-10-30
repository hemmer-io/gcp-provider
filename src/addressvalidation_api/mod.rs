//! Addressvalidation_api Service
//!
//! Auto-generated service module for addressvalidation_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for addressvalidation_api
pub struct Addressvalidation_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Addressvalidation_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get addressvalidation resource handler
    pub fn addressvalidation(&self) -> resources::Addressvalidation<'_> {
        resources::Addressvalidation::new(self.provider)
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
