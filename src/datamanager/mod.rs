//! Datamanager Service
//!
//! Auto-generated service module for datamanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datamanager
pub struct DatamanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DatamanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get request_statu resource handler
    pub fn request_statu(&self) -> resources::Request_statu<'_> {
        resources::Request_statu::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get audience_member resource handler
    pub fn audience_member(&self) -> resources::Audience_member<'_> {
        resources::Audience_member::new(self.provider)
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
