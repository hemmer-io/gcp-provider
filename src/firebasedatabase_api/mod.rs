//! Firebasedatabase_api Service
//!
//! Auto-generated service module for firebasedatabase_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebasedatabase_api
pub struct Firebasedatabase_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebasedatabase_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
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
