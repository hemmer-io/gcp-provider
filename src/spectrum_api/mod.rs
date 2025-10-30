//! Spectrum_api Service
//!
//! Auto-generated service module for spectrum_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for spectrum_api
pub struct Spectrum_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Spectrum_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get paw resource handler
    pub fn paw(&self) -> resources::Paw<'_> {
        resources::Paw::new(self.provider)
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
