//! Serviceusage Service
//!
//! Auto-generated service module for serviceusage

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for serviceusage
pub struct ServiceusageService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServiceusageService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get admin_override resource handler
    pub fn admin_override(&self) -> resources::Admin_override<'_> {
        resources::Admin_override::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get consumer_quota_metric resource handler
    pub fn consumer_quota_metric(&self) -> resources::Consumer_quota_metric<'_> {
        resources::Consumer_quota_metric::new(self.provider)
    }
    /// Get limit resource handler
    pub fn limit(&self) -> resources::Limit<'_> {
        resources::Limit::new(self.provider)
    }
    /// Get consumer_override resource handler
    pub fn consumer_override(&self) -> resources::Consumer_override<'_> {
        resources::Consumer_override::new(self.provider)
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
