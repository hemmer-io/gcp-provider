//! Fcm_api Service
//!
//! Auto-generated service module for fcm_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fcm_api
pub struct Fcm_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Fcm_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
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
