//! Commentanalyzer_api Service
//!
//! Auto-generated service module for commentanalyzer_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for commentanalyzer_api
pub struct Commentanalyzer_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Commentanalyzer_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
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
