//! Domains_api Service
//!
//! Auto-generated service module for domains_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for domains_api
pub struct Domains_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Domains_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get registration resource handler
    pub fn registration(&self) -> resources::Registration<'_> {
        resources::Registration::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
