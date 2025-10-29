//! Digitalassetlinks Service
//!
//! Auto-generated service module for digitalassetlinks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for digitalassetlinks
pub struct DigitalassetlinksService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DigitalassetlinksService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get assetlink resource handler
    pub fn assetlink(&self) -> resources::Assetlink<'_> {
        resources::Assetlink::new(self.provider)
    }
    /// Get statement resource handler
    pub fn statement(&self) -> resources::Statement<'_> {
        resources::Statement::new(self.provider)
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
