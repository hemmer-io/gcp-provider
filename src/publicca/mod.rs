//! Publicca Service
//!
//! Auto-generated service module for publicca

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for publicca
pub struct PubliccaService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PubliccaService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get external_account_key resource handler
    pub fn external_account_key(&self) -> resources::External_account_key<'_> {
        resources::External_account_key::new(self.provider)
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
