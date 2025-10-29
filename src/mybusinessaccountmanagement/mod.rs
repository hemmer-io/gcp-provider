//! Mybusinessaccountmanagement Service
//!
//! Auto-generated service module for mybusinessaccountmanagement

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessaccountmanagement
pub struct MybusinessaccountmanagementService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MybusinessaccountmanagementService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get invitation resource handler
    pub fn invitation(&self) -> resources::Invitation<'_> {
        resources::Invitation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get admin resource handler
    pub fn admin(&self) -> resources::Admin<'_> {
        resources::Admin::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
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
