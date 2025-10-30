//! Recommendationengine_api Service
//!
//! Auto-generated service module for recommendationengine_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for recommendationengine_api
pub struct Recommendationengine_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Recommendationengine_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get user_event resource handler
    pub fn user_event(&self) -> resources::User_event<'_> {
        resources::User_event::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get placement resource handler
    pub fn placement(&self) -> resources::Placement<'_> {
        resources::Placement::new(self.provider)
    }
    /// Get prediction_api_key_registration resource handler
    pub fn prediction_api_key_registration(&self) -> resources::Prediction_api_key_registration<'_> {
        resources::Prediction_api_key_registration::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
    }
    /// Get catalog_item resource handler
    pub fn catalog_item(&self) -> resources::Catalog_item<'_> {
        resources::Catalog_item::new(self.provider)
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
