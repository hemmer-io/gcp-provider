//! Firebasedataconnect Service
//!
//! Auto-generated service module for firebasedataconnect

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebasedataconnect
pub struct FirebasedataconnectService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebasedataconnectService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
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
