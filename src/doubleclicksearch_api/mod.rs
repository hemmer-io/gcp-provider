//! Doubleclicksearch_api Service
//!
//! Auto-generated service module for doubleclicksearch_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for doubleclicksearch_api
pub struct Doubleclicksearch_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Doubleclicksearch_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get saved_column resource handler
    pub fn saved_column(&self) -> resources::Saved_column<'_> {
        resources::Saved_column::new(self.provider)
    }
    /// Get conversion resource handler
    pub fn conversion(&self) -> resources::Conversion<'_> {
        resources::Conversion::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
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
