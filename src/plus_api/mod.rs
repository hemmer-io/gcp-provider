//! Plus_api Service
//!
//! Auto-generated service module for plus_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for plus_api
pub struct Plus_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Plus_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get people resource handler
    pub fn people(&self) -> resources::People<'_> {
        resources::People::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
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
