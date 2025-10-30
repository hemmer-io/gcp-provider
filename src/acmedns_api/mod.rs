//! Acmedns_api Service
//!
//! Auto-generated service module for acmedns_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for acmedns_api
pub struct Acmedns_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Acmedns_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get acme_challenge_set resource handler
    pub fn acme_challenge_set(&self) -> resources::Acme_challenge_set<'_> {
        resources::Acme_challenge_set::new(self.provider)
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
