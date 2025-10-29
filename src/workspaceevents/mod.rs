//! Workspaceevents Service
//!
//! Auto-generated service module for workspaceevents

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaceevents
pub struct WorkspaceeventsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WorkspaceeventsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
