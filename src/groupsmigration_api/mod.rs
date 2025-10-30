//! Groupsmigration_api Service
//!
//! Auto-generated service module for groupsmigration_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for groupsmigration_api
pub struct Groupsmigration_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Groupsmigration_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get archive resource handler
    pub fn archive(&self) -> resources::Archive<'_> {
        resources::Archive::new(self.provider)
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
