//! Datamanager_api Service
//!
//! Auto-generated service module for datamanager_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datamanager_api
pub struct Datamanager_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Datamanager_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get request_statu resource handler
    pub fn request_statu(&self) -> resources::Request_statu<'_> {
        resources::Request_statu::new(self.provider)
    }
    /// Get audience_member resource handler
    pub fn audience_member(&self) -> resources::Audience_member<'_> {
        resources::Audience_member::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
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
