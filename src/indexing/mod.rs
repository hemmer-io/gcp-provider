//! Indexing Service
//!
//! Auto-generated service module for indexing

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for indexing
pub struct IndexingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> IndexingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get url_notification resource handler
    pub fn url_notification(&self) -> resources::Url_notification<'_> {
        resources::Url_notification::new(self.provider)
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
