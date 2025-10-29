//! Commentanalyzer Service
//!
//! Auto-generated service module for commentanalyzer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for commentanalyzer
pub struct CommentanalyzerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CommentanalyzerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
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
