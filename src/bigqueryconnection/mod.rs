//! Bigqueryconnection Service
//!
//! Auto-generated service module for bigqueryconnection

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigqueryconnection
pub struct BigqueryconnectionService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BigqueryconnectionService<'a> {
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
