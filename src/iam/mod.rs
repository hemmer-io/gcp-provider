//! Iam Service
//!
//! Auto-generated service module for iam

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iam
pub struct IamService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> IamService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
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
