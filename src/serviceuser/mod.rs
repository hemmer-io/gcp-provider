//! Serviceuser Service
//!
//! Auto-generated service module for serviceuser

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for serviceuser
pub struct ServiceuserService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServiceuserService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
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
