//! Runtimeconfig_api Service
//!
//! Auto-generated service module for runtimeconfig_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for runtimeconfig_api
pub struct Runtimeconfig_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Runtimeconfig_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get waiter resource handler
    pub fn waiter(&self) -> resources::Waiter<'_> {
        resources::Waiter::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get variable resource handler
    pub fn variable(&self) -> resources::Variable<'_> {
        resources::Variable::new(self.provider)
    }
    /// Get config resource handler
    pub fn config(&self) -> resources::Config<'_> {
        resources::Config::new(self.provider)
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
