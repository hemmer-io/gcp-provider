//! Analyticsdata Service
//!
//! Auto-generated service module for analyticsdata

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for analyticsdata
pub struct AnalyticsdataService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AnalyticsdataService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get audience_export resource handler
    pub fn audience_export(&self) -> resources::Audience_export<'_> {
        resources::Audience_export::new(self.provider)
    }
    /// Get propertie resource handler
    pub fn propertie(&self) -> resources::Propertie<'_> {
        resources::Propertie::new(self.provider)
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
