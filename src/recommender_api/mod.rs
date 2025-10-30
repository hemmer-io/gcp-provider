//! Recommender_api Service
//!
//! Auto-generated service module for recommender_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for recommender_api
pub struct Recommender_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Recommender_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get recommendation resource handler
    pub fn recommendation(&self) -> resources::Recommendation<'_> {
        resources::Recommendation::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get recommender resource handler
    pub fn recommender(&self) -> resources::Recommender<'_> {
        resources::Recommender::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get insight_type resource handler
    pub fn insight_type(&self) -> resources::Insight_type<'_> {
        resources::Insight_type::new(self.provider)
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
