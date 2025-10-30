//! Dialogflow_api Service
//!
//! Auto-generated service module for dialogflow_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dialogflow_api
pub struct Dialogflow_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Dialogflow_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
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
