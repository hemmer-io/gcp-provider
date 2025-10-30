//! Contactcenteraiplatform_api Service
//!
//! Auto-generated service module for contactcenteraiplatform_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for contactcenteraiplatform_api
pub struct Contactcenteraiplatform_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Contactcenteraiplatform_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get contact_center resource handler
    pub fn contact_center(&self) -> resources::Contact_center<'_> {
        resources::Contact_center::new(self.provider)
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
