//! Mybusinesslodging_api Service
//!
//! Auto-generated service module for mybusinesslodging_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinesslodging_api
pub struct Mybusinesslodging_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinesslodging_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get lodging resource handler
    pub fn lodging(&self) -> resources::Lodging<'_> {
        resources::Lodging::new(self.provider)
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
