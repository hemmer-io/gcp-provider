//! Saasservicemgmt Service
//!
//! Auto-generated service module for saasservicemgmt

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for saasservicemgmt
pub struct SaasservicemgmtService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SaasservicemgmtService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get tenant resource handler
    pub fn tenant(&self) -> resources::Tenant<'_> {
        resources::Tenant::new(self.provider)
    }
    /// Get release resource handler
    pub fn release(&self) -> resources::Release<'_> {
        resources::Release::new(self.provider)
    }
    /// Get rollout resource handler
    pub fn rollout(&self) -> resources::Rollout<'_> {
        resources::Rollout::new(self.provider)
    }
    /// Get rollout_kind resource handler
    pub fn rollout_kind(&self) -> resources::Rollout_kind<'_> {
        resources::Rollout_kind::new(self.provider)
    }
    /// Get replications_internal resource handler
    pub fn replications_internal(&self) -> resources::Replications_internal<'_> {
        resources::Replications_internal::new(self.provider)
    }
    /// Get saa resource handler
    pub fn saa(&self) -> resources::Saa<'_> {
        resources::Saa::new(self.provider)
    }
    /// Get unit resource handler
    pub fn unit(&self) -> resources::Unit<'_> {
        resources::Unit::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get unit_kind resource handler
    pub fn unit_kind(&self) -> resources::Unit_kind<'_> {
        resources::Unit_kind::new(self.provider)
    }
    /// Get unit_operation resource handler
    pub fn unit_operation(&self) -> resources::Unit_operation<'_> {
        resources::Unit_operation::new(self.provider)
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
