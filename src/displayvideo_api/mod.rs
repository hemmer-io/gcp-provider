//! Displayvideo_api Service
//!
//! Auto-generated service module for displayvideo_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for displayvideo_api
pub struct Displayvideo_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Displayvideo_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
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
