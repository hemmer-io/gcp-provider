//! Accessapproval Service
//!
//! Auto-generated service module for accessapproval

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for accessapproval
pub struct AccessapprovalService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AccessapprovalService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get approval_request resource handler
    pub fn approval_request(&self) -> resources::Approval_request<'_> {
        resources::Approval_request::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
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
