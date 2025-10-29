//! Marketingplatformadmin Service
//!
//! Auto-generated service module for marketingplatformadmin

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for marketingplatformadmin
pub struct MarketingplatformadminService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MarketingplatformadminService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get analytics_account_link resource handler
    pub fn analytics_account_link(&self) -> resources::Analytics_account_link<'_> {
        resources::Analytics_account_link::new(self.provider)
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
