//! Dns Service
//!
//! Auto-generated service module for dns

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dns
pub struct DnsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DnsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get change resource handler
    pub fn change(&self) -> resources::Change<'_> {
        resources::Change::new(self.provider)
    }
    /// Get dns_key resource handler
    pub fn dns_key(&self) -> resources::Dns_key<'_> {
        resources::Dns_key::new(self.provider)
    }
    /// Get response_policie resource handler
    pub fn response_policie(&self) -> resources::Response_policie<'_> {
        resources::Response_policie::new(self.provider)
    }
    /// Get managed_zone_operation resource handler
    pub fn managed_zone_operation(&self) -> resources::Managed_zone_operation<'_> {
        resources::Managed_zone_operation::new(self.provider)
    }
    /// Get response_policy_rule resource handler
    pub fn response_policy_rule(&self) -> resources::Response_policy_rule<'_> {
        resources::Response_policy_rule::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get resource_record_set resource handler
    pub fn resource_record_set(&self) -> resources::Resource_record_set<'_> {
        resources::Resource_record_set::new(self.provider)
    }
    /// Get managed_zone resource handler
    pub fn managed_zone(&self) -> resources::Managed_zone<'_> {
        resources::Managed_zone::new(self.provider)
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
