//! Playablelocations Service
//!
//! Auto-generated service module for playablelocations

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playablelocations
pub struct PlayablelocationsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PlayablelocationsService<'a> {
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
