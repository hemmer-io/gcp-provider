//! Appstate Service
//!
//! Auto-generated service module for appstate

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appstate
pub struct AppstateService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AppstateService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get state resource handler
    pub fn state(&self) -> resources::State<'_> {
        resources::State::new(self.provider)
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
