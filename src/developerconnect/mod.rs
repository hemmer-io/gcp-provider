//! Developerconnect Service
//!
//! Auto-generated service module for developerconnect

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for developerconnect
pub struct DeveloperconnectService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DeveloperconnectService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get insights_config resource handler
    pub fn insights_config(&self) -> resources::Insights_config<'_> {
        resources::Insights_config::new(self.provider)
    }
    /// Get account_connector resource handler
    pub fn account_connector(&self) -> resources::Account_connector<'_> {
        resources::Account_connector::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get git_repository_link resource handler
    pub fn git_repository_link(&self) -> resources::Git_repository_link<'_> {
        resources::Git_repository_link::new(self.provider)
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
