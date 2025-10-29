//! Webmasters Service
//!
//! Auto-generated service module for webmasters

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for webmasters
pub struct WebmastersService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WebmastersService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get sitemap resource handler
    pub fn sitemap(&self) -> resources::Sitemap<'_> {
        resources::Sitemap::new(self.provider)
    }
    /// Get searchanalytic resource handler
    pub fn searchanalytic(&self) -> resources::Searchanalytic<'_> {
        resources::Searchanalytic::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
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
