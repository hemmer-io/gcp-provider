//! Firebaseappcheck_api Service
//!
//! Auto-generated service module for firebaseappcheck_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseappcheck_api
pub struct Firebaseappcheck_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebaseappcheck_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get recaptcha_config resource handler
    pub fn recaptcha_config(&self) -> resources::Recaptcha_config<'_> {
        resources::Recaptcha_config::new(self.provider)
    }
    /// Get debug_token resource handler
    pub fn debug_token(&self) -> resources::Debug_token<'_> {
        resources::Debug_token::new(self.provider)
    }
    /// Get oauth_client resource handler
    pub fn oauth_client(&self) -> resources::Oauth_client<'_> {
        resources::Oauth_client::new(self.provider)
    }
    /// Get recaptcha_v3_config resource handler
    pub fn recaptcha_v3_config(&self) -> resources::Recaptcha_v3_config<'_> {
        resources::Recaptcha_v3_config::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get app_attest_config resource handler
    pub fn app_attest_config(&self) -> resources::App_attest_config<'_> {
        resources::App_attest_config::new(self.provider)
    }
    /// Get safety_net_config resource handler
    pub fn safety_net_config(&self) -> resources::Safety_net_config<'_> {
        resources::Safety_net_config::new(self.provider)
    }
    /// Get play_integrity_config resource handler
    pub fn play_integrity_config(&self) -> resources::Play_integrity_config<'_> {
        resources::Play_integrity_config::new(self.provider)
    }
    /// Get resource_policie resource handler
    pub fn resource_policie(&self) -> resources::Resource_policie<'_> {
        resources::Resource_policie::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get jwk resource handler
    pub fn jwk(&self) -> resources::Jwk<'_> {
        resources::Jwk::new(self.provider)
    }
    /// Get device_check_config resource handler
    pub fn device_check_config(&self) -> resources::Device_check_config<'_> {
        resources::Device_check_config::new(self.provider)
    }
    /// Get recaptcha_enterprise_config resource handler
    pub fn recaptcha_enterprise_config(&self) -> resources::Recaptcha_enterprise_config<'_> {
        resources::Recaptcha_enterprise_config::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
