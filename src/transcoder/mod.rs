//! Transcoder Service
//!
//! Auto-generated service module for transcoder

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for transcoder
pub struct TranscoderService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TranscoderService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get job_template resource handler
    pub fn job_template(&self) -> resources::Job_template<'_> {
        resources::Job_template::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
