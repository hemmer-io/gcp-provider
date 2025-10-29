//! Civicinfo Service
//!
//! Auto-generated service module for civicinfo

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for civicinfo
pub struct CivicinfoService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CivicinfoService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get election resource handler
    pub fn election(&self) -> resources::Election<'_> {
        resources::Election::new(self.provider)
    }
    /// Get division resource handler
    pub fn division(&self) -> resources::Division<'_> {
        resources::Division::new(self.provider)
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
