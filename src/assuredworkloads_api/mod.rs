//! Assuredworkloads_api Service
//!
//! Auto-generated service module for assuredworkloads_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for assuredworkloads_api
pub struct Assuredworkloads_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Assuredworkloads_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get update resource handler
    pub fn update(&self) -> resources::Update<'_> {
        resources::Update::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get violation resource handler
    pub fn violation(&self) -> resources::Violation<'_> {
        resources::Violation::new(self.provider)
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
