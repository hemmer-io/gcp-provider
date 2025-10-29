//! Gmailpostmastertools Service
//!
//! Auto-generated service module for gmailpostmastertools

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gmailpostmastertools
pub struct GmailpostmastertoolsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GmailpostmastertoolsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get traffic_stat resource handler
    pub fn traffic_stat(&self) -> resources::Traffic_stat<'_> {
        resources::Traffic_stat::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
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
