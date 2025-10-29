//! Policytroubleshooter Service
//!
//! Auto-generated service module for policytroubleshooter

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for policytroubleshooter
pub struct PolicytroubleshooterService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PolicytroubleshooterService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get iam resource handler
    pub fn iam(&self) -> resources::Iam<'_> {
        resources::Iam::new(self.provider)
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
