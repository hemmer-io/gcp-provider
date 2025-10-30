//! Verifiedaccess_api Service
//!
//! Auto-generated service module for verifiedaccess_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for verifiedaccess_api
pub struct Verifiedaccess_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Verifiedaccess_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get challenge resource handler
    pub fn challenge(&self) -> resources::Challenge<'_> {
        resources::Challenge::new(self.provider)
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
