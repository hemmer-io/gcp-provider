//! Cloudtrace Service
//!
//! Auto-generated service module for cloudtrace

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtrace
pub struct CloudtraceService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudtraceService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get trace resource handler
    pub fn trace(&self) -> resources::Trace<'_> {
        resources::Trace::new(self.provider)
    }
    /// Get span resource handler
    pub fn span(&self) -> resources::Span<'_> {
        resources::Span::new(self.provider)
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
