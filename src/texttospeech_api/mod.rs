//! Texttospeech_api Service
//!
//! Auto-generated service module for texttospeech_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for texttospeech_api
pub struct Texttospeech_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Texttospeech_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get voice resource handler
    pub fn voice(&self) -> resources::Voice<'_> {
        resources::Voice::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get text resource handler
    pub fn text(&self) -> resources::Text<'_> {
        resources::Text::new(self.provider)
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
