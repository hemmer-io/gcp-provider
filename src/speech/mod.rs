//! Speech Service
//!
//! Auto-generated service module for speech

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for speech
pub struct SpeechService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SpeechService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get custom_classe resource handler
    pub fn custom_classe(&self) -> resources::Custom_classe<'_> {
        resources::Custom_classe::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get phrase_set resource handler
    pub fn phrase_set(&self) -> resources::Phrase_set<'_> {
        resources::Phrase_set::new(self.provider)
    }
    /// Get speech resource handler
    pub fn speech(&self) -> resources::Speech<'_> {
        resources::Speech::new(self.provider)
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
