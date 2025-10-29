//! Searchconsole Service
//!
//! Auto-generated service module for searchconsole

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for searchconsole
pub struct SearchconsoleService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SearchconsoleService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get index resource handler
    pub fn index(&self) -> resources::Index<'_> {
        resources::Index::new(self.provider)
    }
    /// Get mobile_friendly_test resource handler
    pub fn mobile_friendly_test(&self) -> resources::Mobile_friendly_test<'_> {
        resources::Mobile_friendly_test::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get searchanalytic resource handler
    pub fn searchanalytic(&self) -> resources::Searchanalytic<'_> {
        resources::Searchanalytic::new(self.provider)
    }
    /// Get sitemap resource handler
    pub fn sitemap(&self) -> resources::Sitemap<'_> {
        resources::Sitemap::new(self.provider)
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
