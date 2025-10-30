//! Androidmanagement_api Service
//!
//! Auto-generated service module for androidmanagement_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for androidmanagement_api
pub struct Androidmanagement_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Androidmanagement_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get signup_url resource handler
    pub fn signup_url(&self) -> resources::Signup_url<'_> {
        resources::Signup_url::new(self.provider)
    }
    /// Get enterprise resource handler
    pub fn enterprise(&self) -> resources::Enterprise<'_> {
        resources::Enterprise::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get web_token resource handler
    pub fn web_token(&self) -> resources::Web_token<'_> {
        resources::Web_token::new(self.provider)
    }
    /// Get migration_token resource handler
    pub fn migration_token(&self) -> resources::Migration_token<'_> {
        resources::Migration_token::new(self.provider)
    }
    /// Get provisioning_info resource handler
    pub fn provisioning_info(&self) -> resources::Provisioning_info<'_> {
        resources::Provisioning_info::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get enrollment_token resource handler
    pub fn enrollment_token(&self) -> resources::Enrollment_token<'_> {
        resources::Enrollment_token::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get policie resource handler
    pub fn policie(&self) -> resources::Policie<'_> {
        resources::Policie::new(self.provider)
    }
    /// Get web_app resource handler
    pub fn web_app(&self) -> resources::Web_app<'_> {
        resources::Web_app::new(self.provider)
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
