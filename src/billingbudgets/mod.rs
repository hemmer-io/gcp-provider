//! Billingbudgets Service
//!
//! Auto-generated service module for billingbudgets

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for billingbudgets
pub struct BillingbudgetsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BillingbudgetsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get budget resource handler
    pub fn budget(&self) -> resources::Budget<'_> {
        resources::Budget::new(self.provider)
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
