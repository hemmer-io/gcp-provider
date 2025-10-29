//! Mybusinessqanda Service
//!
//! Auto-generated service module for mybusinessqanda

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessqanda
pub struct MybusinessqandaService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MybusinessqandaService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get question resource handler
    pub fn question(&self) -> resources::Question<'_> {
        resources::Question::new(self.provider)
    }
    /// Get answer resource handler
    pub fn answer(&self) -> resources::Answer<'_> {
        resources::Answer::new(self.provider)
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
