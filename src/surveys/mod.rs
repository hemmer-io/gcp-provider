//! Surveys Service
//!
//! Auto-generated service module for surveys

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for surveys
pub struct SurveysService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> SurveysService<'a> {
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
