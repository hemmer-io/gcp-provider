//! Servicecontrol Service
//!
//! Auto-generated service module for servicecontrol

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for servicecontrol
pub struct ServicecontrolService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ServicecontrolService<'a> {
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
