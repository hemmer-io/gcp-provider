//! Playablelocations_api Service
//!
//! Auto-generated service module for playablelocations_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playablelocations_api
pub struct Playablelocations_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Playablelocations_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get playablelocation resource handler
    pub fn playablelocation(&self) -> resources::Playablelocation<'_> {
        resources::Playablelocation::new(self.provider)
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
