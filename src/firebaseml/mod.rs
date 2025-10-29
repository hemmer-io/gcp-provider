//! Firebaseml Service
//!
//! Auto-generated service module for firebaseml

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseml
pub struct FirebasemlService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebasemlService<'a> {
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
