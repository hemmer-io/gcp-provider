//! Appsactivity_api Service
//!
//! Auto-generated service module for appsactivity_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appsactivity_api
pub struct Appsactivity_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Appsactivity_apiService<'a> {
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
