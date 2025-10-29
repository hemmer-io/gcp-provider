//! Vault Service
//!
//! Auto-generated service module for vault

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vault
pub struct VaultService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VaultService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get saved_querie resource handler
    pub fn saved_querie(&self) -> resources::Saved_querie<'_> {
        resources::Saved_querie::new(self.provider)
    }
    /// Get hold resource handler
    pub fn hold(&self) -> resources::Hold<'_> {
        resources::Hold::new(self.provider)
    }
    /// Get matter resource handler
    pub fn matter(&self) -> resources::Matter<'_> {
        resources::Matter::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
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
