//! Siteverification Service
//!
//! Auto-generated service module for siteverification

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for siteverification
pub struct SiteverificationService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SiteverificationService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get web_resource resource handler
    pub fn web_resource(&self) -> resources::Web_resource<'_> {
        resources::Web_resource::new(self.provider)
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
