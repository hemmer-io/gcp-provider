//! Vision_api Service
//!
//! Auto-generated service module for vision_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vision_api
pub struct Vision_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Vision_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get file resource handler
    pub fn file(&self) -> resources::File<'_> {
        resources::File::new(self.provider)
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
