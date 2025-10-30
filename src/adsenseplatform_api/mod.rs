//! Adsenseplatform_api Service
//!
//! Auto-generated service module for adsenseplatform_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adsenseplatform_api
pub struct Adsenseplatform_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adsenseplatform_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get platform resource handler
    pub fn platform(&self) -> resources::Platform<'_> {
        resources::Platform::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
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
