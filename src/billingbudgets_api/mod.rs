//! Billingbudgets_api Service
//!
//! Auto-generated service module for billingbudgets_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for billingbudgets_api
pub struct Billingbudgets_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Billingbudgets_apiService<'a> {
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
