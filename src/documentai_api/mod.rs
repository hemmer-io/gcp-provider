//! Documentai_api Service
//!
//! Auto-generated service module for documentai_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for documentai_api
pub struct Documentai_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Documentai_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
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
