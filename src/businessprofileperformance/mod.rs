//! Businessprofileperformance Service
//!
//! Auto-generated service module for businessprofileperformance

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for businessprofileperformance
pub struct BusinessprofileperformanceService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BusinessprofileperformanceService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get monthly resource handler
    pub fn monthly(&self) -> resources::Monthly<'_> {
        resources::Monthly::new(self.provider)
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
