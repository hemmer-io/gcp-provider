//! Firebaserules Service
//!
//! Auto-generated service module for firebaserules

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaserules
pub struct FirebaserulesService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebaserulesService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get release resource handler
    pub fn release(&self) -> resources::Release<'_> {
        resources::Release::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get ruleset resource handler
    pub fn ruleset(&self) -> resources::Ruleset<'_> {
        resources::Ruleset::new(self.provider)
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
