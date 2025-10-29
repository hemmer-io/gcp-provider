//! Firebaseremoteconfig Service
//!
//! Auto-generated service module for firebaseremoteconfig

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseremoteconfig
pub struct FirebaseremoteconfigService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebaseremoteconfigService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
