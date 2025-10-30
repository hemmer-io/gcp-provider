//! Mybusinessplaceactions_api Service
//!
//! Auto-generated service module for mybusinessplaceactions_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessplaceactions_api
pub struct Mybusinessplaceactions_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinessplaceactions_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get place_action_type_metadata resource handler
    pub fn place_action_type_metadata(&self) -> resources::Place_action_type_metadata<'_> {
        resources::Place_action_type_metadata::new(self.provider)
    }
    /// Get place_action_link resource handler
    pub fn place_action_link(&self) -> resources::Place_action_link<'_> {
        resources::Place_action_link::new(self.provider)
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
