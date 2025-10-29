//! Firebaseapphosting Service
//!
//! Auto-generated service module for firebaseapphosting

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseapphosting
pub struct FirebaseapphostingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebaseapphostingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get build resource handler
    pub fn build(&self) -> resources::Build<'_> {
        resources::Build::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get backend resource handler
    pub fn backend(&self) -> resources::Backend<'_> {
        resources::Backend::new(self.provider)
    }
    /// Get rollout resource handler
    pub fn rollout(&self) -> resources::Rollout<'_> {
        resources::Rollout::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get traffic resource handler
    pub fn traffic(&self) -> resources::Traffic<'_> {
        resources::Traffic::new(self.provider)
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
