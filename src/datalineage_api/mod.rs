//! Datalineage_api Service
//!
//! Auto-generated service module for datalineage_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datalineage_api
pub struct Datalineage_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Datalineage_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get lineage_event resource handler
    pub fn lineage_event(&self) -> resources::Lineage_event<'_> {
        resources::Lineage_event::new(self.provider)
    }
    /// Get processe resource handler
    pub fn processe(&self) -> resources::Processe<'_> {
        resources::Processe::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get run resource handler
    pub fn run(&self) -> resources::Run<'_> {
        resources::Run::new(self.provider)
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
