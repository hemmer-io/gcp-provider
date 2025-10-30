//! Language_api Service
//!
//! Auto-generated service module for language_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for language_api
pub struct Language_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Language_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
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
