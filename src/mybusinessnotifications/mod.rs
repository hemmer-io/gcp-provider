//! Mybusinessnotifications Service
//!
//! Auto-generated service module for mybusinessnotifications

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessnotifications
pub struct MybusinessnotificationsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MybusinessnotificationsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
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
