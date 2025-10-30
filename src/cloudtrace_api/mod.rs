//! Cloudtrace_api Service
//!
//! Auto-generated service module for cloudtrace_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtrace_api
pub struct Cloudtrace_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudtrace_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get span resource handler
    pub fn span(&self) -> resources::Span<'_> {
        resources::Span::new(self.provider)
    }
    /// Get trace resource handler
    pub fn trace(&self) -> resources::Trace<'_> {
        resources::Trace::new(self.provider)
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
