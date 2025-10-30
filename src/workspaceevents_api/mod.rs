//! Workspaceevents_api Service
//!
//! Auto-generated service module for workspaceevents_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaceevents_api
pub struct Workspaceevents_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Workspaceevents_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
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
