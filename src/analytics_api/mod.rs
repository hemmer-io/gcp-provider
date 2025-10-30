//! Analytics_api Service
//!
//! Auto-generated service module for analytics_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analytics_api
pub struct Analytics_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Analytics_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get goal resource handler
    pub fn goal(&self) -> resources::Goal<'_> {
        resources::Goal::new(self.provider)
    }
    /// Get webpropertie resource handler
    pub fn webpropertie(&self) -> resources::Webpropertie<'_> {
        resources::Webpropertie::new(self.provider)
    }
    /// Get segment resource handler
    pub fn segment(&self) -> resources::Segment<'_> {
        resources::Segment::new(self.provider)
    }
    /// Get data resource handler
    pub fn data(&self) -> resources::Data<'_> {
        resources::Data::new(self.provider)
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
