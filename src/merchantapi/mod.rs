//! Merchantapi Service
//!
//! Auto-generated service module for merchantapi

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for merchantapi
pub struct MerchantapiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MerchantapiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
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
