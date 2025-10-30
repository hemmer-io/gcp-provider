//! Civicinfo_api Service
//!
//! Auto-generated service module for civicinfo_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for civicinfo_api
pub struct Civicinfo_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Civicinfo_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get division resource handler
    pub fn division(&self) -> resources::Division<'_> {
        resources::Division::new(self.provider)
    }
    /// Get election resource handler
    pub fn election(&self) -> resources::Election<'_> {
        resources::Election::new(self.provider)
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
