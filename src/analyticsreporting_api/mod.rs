//! Analyticsreporting_api Service
//!
//! Auto-generated service module for analyticsreporting_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analyticsreporting_api
pub struct Analyticsreporting_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Analyticsreporting_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get user_activity resource handler
    pub fn user_activity(&self) -> resources::User_activity<'_> {
        resources::User_activity::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
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
