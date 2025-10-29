//! Drivelabels Service
//!
//! Auto-generated service module for drivelabels

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for drivelabels
pub struct DrivelabelsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DrivelabelsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get label resource handler
    pub fn label(&self) -> resources::Label<'_> {
        resources::Label::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get lock resource handler
    pub fn lock(&self) -> resources::Lock<'_> {
        resources::Lock::new(self.provider)
    }
    /// Get limit resource handler
    pub fn limit(&self) -> resources::Limit<'_> {
        resources::Limit::new(self.provider)
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
