//! Chromewebstore_api Service
//!
//! Auto-generated service module for chromewebstore_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chromewebstore_api
pub struct Chromewebstore_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Chromewebstore_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get item resource handler
    pub fn item(&self) -> resources::Item<'_> {
        resources::Item::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
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
