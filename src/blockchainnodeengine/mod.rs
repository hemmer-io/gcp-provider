//! Blockchainnodeengine Service
//!
//! Auto-generated service module for blockchainnodeengine

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for blockchainnodeengine
pub struct BlockchainnodeengineService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BlockchainnodeengineService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get blockchain_node resource handler
    pub fn blockchain_node(&self) -> resources::Blockchain_node<'_> {
        resources::Blockchain_node::new(self.provider)
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
