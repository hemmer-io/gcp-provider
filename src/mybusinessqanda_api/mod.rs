//! Mybusinessqanda_api Service
//!
//! Auto-generated service module for mybusinessqanda_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessqanda_api
pub struct Mybusinessqanda_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinessqanda_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get answer resource handler
    pub fn answer(&self) -> resources::Answer<'_> {
        resources::Answer::new(self.provider)
    }
    /// Get question resource handler
    pub fn question(&self) -> resources::Question<'_> {
        resources::Question::new(self.provider)
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
