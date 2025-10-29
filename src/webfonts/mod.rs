//! Webfonts Service
//!
//! Auto-generated service module for webfonts

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for webfonts
pub struct WebfontsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WebfontsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get webfont resource handler
    pub fn webfont(&self) -> resources::Webfont<'_> {
        resources::Webfont::new(self.provider)
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
