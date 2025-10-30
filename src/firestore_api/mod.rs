//! Firestore_api Service
//!
//! Auto-generated service module for firestore_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firestore_api
pub struct Firestore_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firestore_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
    }
    /// Get indexe resource handler
    pub fn indexe(&self) -> resources::Indexe<'_> {
        resources::Indexe::new(self.provider)
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
