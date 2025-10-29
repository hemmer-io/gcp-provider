//! Datastream Service
//!
//! Auto-generated service module for datastream

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datastream
pub struct DatastreamService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DatastreamService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get connection_profile resource handler
    pub fn connection_profile(&self) -> resources::Connection_profile<'_> {
        resources::Connection_profile::new(self.provider)
    }
    /// Get stream resource handler
    pub fn stream(&self) -> resources::Stream<'_> {
        resources::Stream::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get private_connection resource handler
    pub fn private_connection(&self) -> resources::Private_connection<'_> {
        resources::Private_connection::new(self.provider)
    }
    /// Get object resource handler
    pub fn object(&self) -> resources::Object<'_> {
        resources::Object::new(self.provider)
    }
    /// Get route resource handler
    pub fn route(&self) -> resources::Route<'_> {
        resources::Route::new(self.provider)
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
