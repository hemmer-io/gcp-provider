//! Chromemanagement Service
//!
//! Auto-generated service module for chromemanagement

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chromemanagement
pub struct ChromemanagementService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ChromemanagementService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get command resource handler
    pub fn command(&self) -> resources::Command<'_> {
        resources::Command::new(self.provider)
    }
    /// Get web resource handler
    pub fn web(&self) -> resources::Web<'_> {
        resources::Web::new(self.provider)
    }
    /// Get third_party_profile_user resource handler
    pub fn third_party_profile_user(&self) -> resources::Third_party_profile_user<'_> {
        resources::Third_party_profile_user::new(self.provider)
    }
    /// Get android resource handler
    pub fn android(&self) -> resources::Android<'_> {
        resources::Android::new(self.provider)
    }
    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
    }
    /// Get chrome resource handler
    pub fn chrome(&self) -> resources::Chrome<'_> {
        resources::Chrome::new(self.provider)
    }
    /// Get notification_config resource handler
    pub fn notification_config(&self) -> resources::Notification_config<'_> {
        resources::Notification_config::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get certificate_provisioning_processe resource handler
    pub fn certificate_provisioning_processe(&self) -> resources::Certificate_provisioning_processe<'_> {
        resources::Certificate_provisioning_processe::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
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
