//! Sourcerepo_api Service
//!
//! Auto-generated service module for sourcerepo_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sourcerepo_api
pub struct Sourcerepo_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sourcerepo_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get repo resource handler
    pub fn repo(&self) -> resources::Repo<'_> {
        resources::Repo::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
