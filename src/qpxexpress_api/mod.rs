//! Qpxexpress_api Service
//!
//! Auto-generated service module for qpxexpress_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for qpxexpress_api
pub struct Qpxexpress_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Qpxexpress_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get trip resource handler
    pub fn trip(&self) -> resources::Trip<'_> {
        resources::Trip::new(self.provider)
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
