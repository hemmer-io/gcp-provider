//! Sts_api Service
//!
//! Auto-generated service module for sts_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sts_api
pub struct Sts_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sts_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get st resource handler
    pub fn st(&self) -> resources::St<'_> {
        resources::St::new(self.provider)
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
