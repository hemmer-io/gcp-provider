//! Cloudasset_api Service
//!
//! Auto-generated service module for cloudasset_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudasset_api
pub struct Cloudasset_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudasset_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get iam_policie resource handler
    pub fn iam_policie(&self) -> resources::Iam_policie<'_> {
        resources::Iam_policie::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
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
