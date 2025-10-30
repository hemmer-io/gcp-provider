//! Meet_api Service
//!
//! Auto-generated service module for meet_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for meet_api
pub struct Meet_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Meet_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get recording resource handler
    pub fn recording(&self) -> resources::Recording<'_> {
        resources::Recording::new(self.provider)
    }
    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
    }
    /// Get transcript resource handler
    pub fn transcript(&self) -> resources::Transcript<'_> {
        resources::Transcript::new(self.provider)
    }
    /// Get participant_session resource handler
    pub fn participant_session(&self) -> resources::Participant_session<'_> {
        resources::Participant_session::new(self.provider)
    }
    /// Get conference_record resource handler
    pub fn conference_record(&self) -> resources::Conference_record<'_> {
        resources::Conference_record::new(self.provider)
    }
    /// Get entrie resource handler
    pub fn entrie(&self) -> resources::Entrie<'_> {
        resources::Entrie::new(self.provider)
    }
    /// Get participant resource handler
    pub fn participant(&self) -> resources::Participant<'_> {
        resources::Participant::new(self.provider)
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
