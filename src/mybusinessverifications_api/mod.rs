//! Mybusinessverifications_api Service
//!
//! Auto-generated service module for mybusinessverifications_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessverifications_api
pub struct Mybusinessverifications_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinessverifications_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get verification_token resource handler
    pub fn verification_token(&self) -> resources::Verification_token<'_> {
        resources::Verification_token::new(self.provider)
    }
    /// Get verification resource handler
    pub fn verification(&self) -> resources::Verification<'_> {
        resources::Verification::new(self.provider)
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
