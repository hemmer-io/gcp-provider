//! Iap Service
//!
//! Auto-generated service module for iap

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iap
pub struct IapService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> IapService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get iap resource handler
    pub fn iap(&self) -> resources::Iap<'_> {
        resources::Iap::new(self.provider)
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
