//! Cloudlocationfinder Service
//!
//! Auto-generated service module for cloudlocationfinder

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudlocationfinder
pub struct CloudlocationfinderService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudlocationfinderService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get cloud_location resource handler
    pub fn cloud_location(&self) -> resources::Cloud_location<'_> {
        resources::Cloud_location::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
