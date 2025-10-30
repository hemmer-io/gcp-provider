//! Cloudsupport_api Service
//!
//! Auto-generated service module for cloudsupport_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudsupport_api
pub struct Cloudsupport_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudsupport_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get case resource handler
    pub fn case(&self) -> resources::Case<'_> {
        resources::Case::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get case_classification resource handler
    pub fn case_classification(&self) -> resources::Case_classification<'_> {
        resources::Case_classification::new(self.provider)
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
