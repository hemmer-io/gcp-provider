//! Webmasters_api Service
//!
//! Auto-generated service module for webmasters_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for webmasters_api
pub struct Webmasters_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Webmasters_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get sitemap resource handler
    pub fn sitemap(&self) -> resources::Sitemap<'_> {
        resources::Sitemap::new(self.provider)
    }
    /// Get searchanalytic resource handler
    pub fn searchanalytic(&self) -> resources::Searchanalytic<'_> {
        resources::Searchanalytic::new(self.provider)
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
