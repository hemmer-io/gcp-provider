//! Bigqueryconnection_api Service
//!
//! Auto-generated service module for bigqueryconnection_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigqueryconnection_api
pub struct Bigqueryconnection_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bigqueryconnection_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
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
