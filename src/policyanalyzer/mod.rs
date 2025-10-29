//! Policyanalyzer Service
//!
//! Auto-generated service module for policyanalyzer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for policyanalyzer
pub struct PolicyanalyzerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PolicyanalyzerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get activitie resource handler
    pub fn activitie(&self) -> resources::Activitie<'_> {
        resources::Activitie::new(self.provider)
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
