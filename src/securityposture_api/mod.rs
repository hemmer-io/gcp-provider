//! Securityposture_api Service
//!
//! Auto-generated service module for securityposture_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for securityposture_api
pub struct Securityposture_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Securityposture_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get posture_template resource handler
    pub fn posture_template(&self) -> resources::Posture_template<'_> {
        resources::Posture_template::new(self.provider)
    }
    /// Get posture_deployment resource handler
    pub fn posture_deployment(&self) -> resources::Posture_deployment<'_> {
        resources::Posture_deployment::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get posture resource handler
    pub fn posture(&self) -> resources::Posture<'_> {
        resources::Posture::new(self.provider)
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
