//! Serviceconsumermanagement Service
//!
//! Auto-generated service module for serviceconsumermanagement

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for serviceconsumermanagement
pub struct ServiceconsumermanagementService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServiceconsumermanagementService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get limit resource handler
    pub fn limit(&self) -> resources::Limit<'_> {
        resources::Limit::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get consumer_quota_metric resource handler
    pub fn consumer_quota_metric(&self) -> resources::Consumer_quota_metric<'_> {
        resources::Consumer_quota_metric::new(self.provider)
    }
    /// Get producer_quota_policie resource handler
    pub fn producer_quota_policie(&self) -> resources::Producer_quota_policie<'_> {
        resources::Producer_quota_policie::new(self.provider)
    }
    /// Get producer_override resource handler
    pub fn producer_override(&self) -> resources::Producer_override<'_> {
        resources::Producer_override::new(self.provider)
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
