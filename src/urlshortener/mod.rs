//! Urlshortener Service
//!
//! Auto-generated service module for urlshortener

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for urlshortener
pub struct UrlshortenerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> UrlshortenerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get url resource handler
    pub fn url(&self) -> resources::Url<'_> {
        resources::Url::new(self.provider)
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
