//! Dialogflow Service
//!
//! Auto-generated service module for dialogflow

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dialogflow
pub struct DialogflowService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DialogflowService<'a> {
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
