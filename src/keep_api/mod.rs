//! Keep_api Service
//!
//! Auto-generated service module for keep_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for keep_api
pub struct Keep_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Keep_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get note resource handler
    pub fn note(&self) -> resources::Note<'_> {
        resources::Note::new(self.provider)
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
