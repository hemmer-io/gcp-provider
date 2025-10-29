//! Slides Service
//!
//! Auto-generated service module for slides

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for slides
pub struct SlidesService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SlidesService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get presentation resource handler
    pub fn presentation(&self) -> resources::Presentation<'_> {
        resources::Presentation::new(self.provider)
    }
    /// Get page resource handler
    pub fn page(&self) -> resources::Page<'_> {
        resources::Page::new(self.provider)
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
