//! Youtubereporting_api Service
//!
//! Auto-generated service module for youtubereporting_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for youtubereporting_api
pub struct Youtubereporting_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Youtubereporting_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get report_type resource handler
    pub fn report_type(&self) -> resources::Report_type<'_> {
        resources::Report_type::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
