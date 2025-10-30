//! Analyticsdata_api Service
//!
//! Auto-generated service module for analyticsdata_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analyticsdata_api
pub struct Analyticsdata_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Analyticsdata_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get propertie resource handler
    pub fn propertie(&self) -> resources::Propertie<'_> {
        resources::Propertie::new(self.provider)
    }
    /// Get audience_export resource handler
    pub fn audience_export(&self) -> resources::Audience_export<'_> {
        resources::Audience_export::new(self.provider)
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
