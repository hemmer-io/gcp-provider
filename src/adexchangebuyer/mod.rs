//! Adexchangebuyer Service
//!
//! Auto-generated service module for adexchangebuyer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adexchangebuyer
pub struct AdexchangebuyerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AdexchangebuyerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get creative resource handler
    pub fn creative(&self) -> resources::Creative<'_> {
        resources::Creative::new(self.provider)
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
