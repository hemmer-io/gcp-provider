//! Playcustomapp Service
//!
//! Auto-generated service module for playcustomapp

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playcustomapp
pub struct PlaycustomappService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PlaycustomappService<'a> {
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
