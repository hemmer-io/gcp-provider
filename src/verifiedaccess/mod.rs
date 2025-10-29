//! Verifiedaccess Service
//!
//! Auto-generated service module for verifiedaccess

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for verifiedaccess
pub struct VerifiedaccessService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VerifiedaccessService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get challenge resource handler
    pub fn challenge(&self) -> resources::Challenge<'_> {
        resources::Challenge::new(self.provider)
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
