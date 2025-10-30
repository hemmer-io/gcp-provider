//! Orgpolicy_api Service
//!
//! Auto-generated service module for orgpolicy_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for orgpolicy_api
pub struct Orgpolicy_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Orgpolicy_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get constraint resource handler
    pub fn constraint(&self) -> resources::Constraint<'_> {
        resources::Constraint::new(self.provider)
    }
    /// Get custom_constraint resource handler
    pub fn custom_constraint(&self) -> resources::Custom_constraint<'_> {
        resources::Custom_constraint::new(self.provider)
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
