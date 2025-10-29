//! Checks Service
//!
//! Auto-generated service module for checks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for checks
pub struct ChecksService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ChecksService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get aisafety resource handler
    pub fn aisafety(&self) -> resources::Aisafety<'_> {
        resources::Aisafety::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get scan resource handler
    pub fn scan(&self) -> resources::Scan<'_> {
        resources::Scan::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
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
