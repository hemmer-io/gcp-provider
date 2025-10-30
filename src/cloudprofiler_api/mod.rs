//! Cloudprofiler_api Service
//!
//! Auto-generated service module for cloudprofiler_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudprofiler_api
pub struct Cloudprofiler_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudprofiler_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
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
