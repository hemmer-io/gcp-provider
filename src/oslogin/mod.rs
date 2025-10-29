//! Oslogin Service
//!
//! Auto-generated service module for oslogin

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for oslogin
pub struct OsloginService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> OsloginService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get ssh_public_key resource handler
    pub fn ssh_public_key(&self) -> resources::Ssh_public_key<'_> {
        resources::Ssh_public_key::new(self.provider)
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
