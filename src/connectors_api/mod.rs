//! Connectors_api Service
//!
//! Auto-generated service module for connectors_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connectors_api
pub struct Connectors_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Connectors_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get tool resource handler
    pub fn tool(&self) -> resources::Tool<'_> {
        resources::Tool::new(self.provider)
    }
    /// Get entity_type resource handler
    pub fn entity_type(&self) -> resources::Entity_type<'_> {
        resources::Entity_type::new(self.provider)
    }
    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get entitie resource handler
    pub fn entitie(&self) -> resources::Entitie<'_> {
        resources::Entitie::new(self.provider)
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
