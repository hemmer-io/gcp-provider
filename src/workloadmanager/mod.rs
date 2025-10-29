//! Workloadmanager Service
//!
//! Auto-generated service module for workloadmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workloadmanager
pub struct WorkloadmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WorkloadmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get result resource handler
    pub fn result(&self) -> resources::Result<'_> {
        resources::Result::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
    }
    /// Get scanned_resource resource handler
    pub fn scanned_resource(&self) -> resources::Scanned_resource<'_> {
        resources::Scanned_resource::new(self.provider)
    }
    /// Get discoveredprofile resource handler
    pub fn discoveredprofile(&self) -> resources::Discoveredprofile<'_> {
        resources::Discoveredprofile::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
