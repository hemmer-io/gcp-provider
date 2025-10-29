//! Secretmanager Service
//!
//! Auto-generated service module for secretmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for secretmanager
pub struct SecretmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SecretmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get secret resource handler
    pub fn secret(&self) -> resources::Secret<'_> {
        resources::Secret::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
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
