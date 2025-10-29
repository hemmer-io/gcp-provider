//! Doubleclickbidmanager Service
//!
//! Auto-generated service module for doubleclickbidmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for doubleclickbidmanager
pub struct DoubleclickbidmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DoubleclickbidmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get querie resource handler
    pub fn querie(&self) -> resources::Querie<'_> {
        resources::Querie::new(self.provider)
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
