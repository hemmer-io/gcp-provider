//! Apikeys Service
//!
//! Auto-generated service module for apikeys

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apikeys
pub struct ApikeysService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ApikeysService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get key resource handler
    pub fn key(&self) -> resources::Key<'_> {
        resources::Key::new(self.provider)
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
