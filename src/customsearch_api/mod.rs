//! Customsearch_api Service
//!
//! Auto-generated service module for customsearch_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for customsearch_api
pub struct Customsearch_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Customsearch_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get cse resource handler
    pub fn cse(&self) -> resources::Cse<'_> {
        resources::Cse::new(self.provider)
    }
    /// Get siterestrict resource handler
    pub fn siterestrict(&self) -> resources::Siterestrict<'_> {
        resources::Siterestrict::new(self.provider)
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
