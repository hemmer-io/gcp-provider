//! Adexchangebuyer_api Service
//!
//! Auto-generated service module for adexchangebuyer_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adexchangebuyer_api
pub struct Adexchangebuyer_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adexchangebuyer_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get creative resource handler
    pub fn creative(&self) -> resources::Creative<'_> {
        resources::Creative::new(self.provider)
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
