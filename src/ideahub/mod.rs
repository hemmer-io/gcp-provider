//! Ideahub Service
//!
//! Auto-generated service module for ideahub

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ideahub
pub struct IdeahubService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> IdeahubService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get idea resource handler
    pub fn idea(&self) -> resources::Idea<'_> {
        resources::Idea::new(self.provider)
    }
    /// Get topic_state resource handler
    pub fn topic_state(&self) -> resources::Topic_state<'_> {
        resources::Topic_state::new(self.provider)
    }
    /// Get idea_state resource handler
    pub fn idea_state(&self) -> resources::Idea_state<'_> {
        resources::Idea_state::new(self.provider)
    }
    /// Get locale resource handler
    pub fn locale(&self) -> resources::Locale<'_> {
        resources::Locale::new(self.provider)
    }
    /// Get idea_activitie resource handler
    pub fn idea_activitie(&self) -> resources::Idea_activitie<'_> {
        resources::Idea_activitie::new(self.provider)
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
