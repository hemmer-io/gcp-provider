//! Oauth2 Service
//!
//! Auto-generated service module for oauth2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for oauth2
pub struct Oauth2Service<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Oauth2Service<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get userinfo resource handler
    pub fn userinfo(&self) -> resources::Userinfo<'_> {
        resources::Userinfo::new(self.provider)
    }
    /// Get me resource handler
    pub fn me(&self) -> resources::Me<'_> {
        resources::Me::new(self.provider)
    }
    /// Get oauth2 resource handler
    pub fn oauth2(&self) -> resources::Oauth2<'_> {
        resources::Oauth2::new(self.provider)
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
