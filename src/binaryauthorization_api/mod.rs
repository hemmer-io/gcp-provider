//! Binaryauthorization_api Service
//!
//! Auto-generated service module for binaryauthorization_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for binaryauthorization_api
pub struct Binaryauthorization_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Binaryauthorization_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get systempolicy resource handler
    pub fn systempolicy(&self) -> resources::Systempolicy<'_> {
        resources::Systempolicy::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get attestor resource handler
    pub fn attestor(&self) -> resources::Attestor<'_> {
        resources::Attestor::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
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
