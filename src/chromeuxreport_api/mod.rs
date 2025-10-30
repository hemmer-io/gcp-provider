//! Chromeuxreport_api Service
//!
//! Auto-generated service module for chromeuxreport_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chromeuxreport_api
pub struct Chromeuxreport_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Chromeuxreport_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get record resource handler
    pub fn record(&self) -> resources::Record<'_> {
        resources::Record::new(self.provider)
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
