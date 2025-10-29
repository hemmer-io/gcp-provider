//! Cloudasset Service
//!
//! Auto-generated service module for cloudasset

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudasset
pub struct CloudassetService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudassetService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get iam_policie resource handler
    pub fn iam_policie(&self) -> resources::Iam_policie<'_> {
        resources::Iam_policie::new(self.provider)
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
