//! Reseller Service
//!
//! Auto-generated service module for reseller

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for reseller
pub struct ResellerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ResellerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get resellernotify resource handler
    pub fn resellernotify(&self) -> resources::Resellernotify<'_> {
        resources::Resellernotify::new(self.provider)
    }
    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
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
