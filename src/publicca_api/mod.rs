//! Publicca_api Service
//!
//! Auto-generated service module for publicca_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for publicca_api
pub struct Publicca_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Publicca_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get external_account_key resource handler
    pub fn external_account_key(&self) -> resources::External_account_key<'_> {
        resources::External_account_key::new(self.provider)
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
