//! Groupssettings_api Service
//!
//! Auto-generated service module for groupssettings_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for groupssettings_api
pub struct Groupssettings_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Groupssettings_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
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
