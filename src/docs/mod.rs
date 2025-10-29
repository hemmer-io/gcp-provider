//! Docs Service
//!
//! Auto-generated service module for docs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for docs
pub struct DocsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DocsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
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
