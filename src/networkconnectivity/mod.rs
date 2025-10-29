//! Networkconnectivity Service
//!
//! Auto-generated service module for networkconnectivity

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networkconnectivity
pub struct NetworkconnectivityService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> NetworkconnectivityService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get hub resource handler
    pub fn hub(&self) -> resources::Hub<'_> {
        resources::Hub::new(self.provider)
    }
    /// Get spoke resource handler
    pub fn spoke(&self) -> resources::Spoke<'_> {
        resources::Spoke::new(self.provider)
    }
    /// Get internal_range resource handler
    pub fn internal_range(&self) -> resources::Internal_range<'_> {
        resources::Internal_range::new(self.provider)
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
