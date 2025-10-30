//! Policysimulator_api Service
//!
//! Auto-generated service module for policysimulator_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for policysimulator_api
pub struct Policysimulator_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Policysimulator_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
