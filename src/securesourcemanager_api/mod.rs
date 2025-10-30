//! Securesourcemanager_api Service
//!
//! Auto-generated service module for securesourcemanager_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for securesourcemanager_api
pub struct Securesourcemanager_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Securesourcemanager_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get pull_request resource handler
    pub fn pull_request(&self) -> resources::Pull_request<'_> {
        resources::Pull_request::new(self.provider)
    }
    /// Get pull_request_comment resource handler
    pub fn pull_request_comment(&self) -> resources::Pull_request_comment<'_> {
        resources::Pull_request_comment::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get branch_rule resource handler
    pub fn branch_rule(&self) -> resources::Branch_rule<'_> {
        resources::Branch_rule::new(self.provider)
    }
    /// Get issue resource handler
    pub fn issue(&self) -> resources::Issue<'_> {
        resources::Issue::new(self.provider)
    }
    /// Get hook resource handler
    pub fn hook(&self) -> resources::Hook<'_> {
        resources::Hook::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get repositorie resource handler
    pub fn repositorie(&self) -> resources::Repositorie<'_> {
        resources::Repositorie::new(self.provider)
    }
    /// Get issue_comment resource handler
    pub fn issue_comment(&self) -> resources::Issue_comment<'_> {
        resources::Issue_comment::new(self.provider)
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
