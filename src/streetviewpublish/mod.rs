//! Streetviewpublish Service
//!
//! Auto-generated service module for streetviewpublish

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for streetviewpublish
pub struct StreetviewpublishService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> StreetviewpublishService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get photo_sequence resource handler
    pub fn photo_sequence(&self) -> resources::Photo_sequence<'_> {
        resources::Photo_sequence::new(self.provider)
    }
    /// Get photo resource handler
    pub fn photo(&self) -> resources::Photo<'_> {
        resources::Photo::new(self.provider)
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
