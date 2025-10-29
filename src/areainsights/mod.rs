//! Areainsights Service
//!
//! Auto-generated service module for areainsights

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for areainsights
pub struct AreainsightsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AreainsightsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get areainsight resource handler
    pub fn areainsight(&self) -> resources::Areainsight<'_> {
        resources::Areainsight::new(self.provider)
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
