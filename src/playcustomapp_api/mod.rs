//! Playcustomapp_api Service
//!
//! Auto-generated service module for playcustomapp_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playcustomapp_api
pub struct Playcustomapp_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Playcustomapp_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get custom_app resource handler
    pub fn custom_app(&self) -> resources::Custom_app<'_> {
        resources::Custom_app::new(self.provider)
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
