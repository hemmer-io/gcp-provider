//! Streetviewpublish_api Service
//!
//! Auto-generated service module for streetviewpublish_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for streetviewpublish_api
pub struct Streetviewpublish_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Streetviewpublish_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get photo resource handler
    pub fn photo(&self) -> resources::Photo<'_> {
        resources::Photo::new(self.provider)
    }
    /// Get photo_sequence resource handler
    pub fn photo_sequence(&self) -> resources::Photo_sequence<'_> {
        resources::Photo_sequence::new(self.provider)
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
