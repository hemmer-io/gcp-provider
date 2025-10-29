//! Cloudcontrolspartner Service
//!
//! Auto-generated service module for cloudcontrolspartner

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudcontrolspartner
pub struct CloudcontrolspartnerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudcontrolspartnerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get access_approval_request resource handler
    pub fn access_approval_request(&self) -> resources::Access_approval_request<'_> {
        resources::Access_approval_request::new(self.provider)
    }
    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get violation resource handler
    pub fn violation(&self) -> resources::Violation<'_> {
        resources::Violation::new(self.provider)
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
