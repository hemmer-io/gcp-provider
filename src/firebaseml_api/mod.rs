//! Firebaseml_api Service
//!
//! Auto-generated service module for firebaseml_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseml_api
pub struct Firebaseml_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebaseml_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
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
