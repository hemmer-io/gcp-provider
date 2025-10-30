//! Consumersurveys_api Service
//!
//! Auto-generated service module for consumersurveys_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for consumersurveys_api
pub struct Consumersurveys_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Consumersurveys_apiService<'a> {
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
    /// Get mobileapppanel resource handler
    pub fn mobileapppanel(&self) -> resources::Mobileapppanel<'_> {
        resources::Mobileapppanel::new(self.provider)
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
