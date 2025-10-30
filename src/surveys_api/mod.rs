//! Surveys_api Service
//!
//! Auto-generated service module for surveys_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for surveys_api
pub struct Surveys_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Surveys_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get result resource handler
    pub fn result(&self) -> resources::Result<'_> {
        resources::Result::new(self.provider)
    }
    /// Get survey resource handler
    pub fn survey(&self) -> resources::Survey<'_> {
        resources::Survey::new(self.provider)
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
