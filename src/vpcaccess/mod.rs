//! Vpcaccess Service
//!
//! Auto-generated service module for vpcaccess

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vpcaccess
pub struct VpcaccessService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VpcaccessService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
