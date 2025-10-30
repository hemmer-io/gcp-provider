//! Firebasestorage_api Service
//!
//! Auto-generated service module for firebasestorage_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebasestorage_api
pub struct Firebasestorage_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebasestorage_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get default_bucket resource handler
    pub fn default_bucket(&self) -> resources::Default_bucket<'_> {
        resources::Default_bucket::new(self.provider)
    }
    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
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
