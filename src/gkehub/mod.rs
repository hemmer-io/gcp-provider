//! Gkehub Service
//!
//! Auto-generated service module for gkehub

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gkehub
pub struct GkehubService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GkehubService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get feature resource handler
    pub fn feature(&self) -> resources::Feature<'_> {
        resources::Feature::new(self.provider)
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
