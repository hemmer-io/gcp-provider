//! Webrisk Service
//!
//! Auto-generated service module for webrisk

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for webrisk
pub struct WebriskService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WebriskService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get submission resource handler
    pub fn submission(&self) -> resources::Submission<'_> {
        resources::Submission::new(self.provider)
    }
    /// Get hashe resource handler
    pub fn hashe(&self) -> resources::Hashe<'_> {
        resources::Hashe::new(self.provider)
    }
    /// Get threat_list resource handler
    pub fn threat_list(&self) -> resources::Threat_list<'_> {
        resources::Threat_list::new(self.provider)
    }
    /// Get uri resource handler
    pub fn uri(&self) -> resources::Uri<'_> {
        resources::Uri::new(self.provider)
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
