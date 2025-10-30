//! Businessprofileperformance_api Service
//!
//! Auto-generated service module for businessprofileperformance_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for businessprofileperformance_api
pub struct Businessprofileperformance_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Businessprofileperformance_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get monthly resource handler
    pub fn monthly(&self) -> resources::Monthly<'_> {
        resources::Monthly::new(self.provider)
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
