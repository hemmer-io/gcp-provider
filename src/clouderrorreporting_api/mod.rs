//! Clouderrorreporting_api Service
//!
//! Auto-generated service module for clouderrorreporting_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for clouderrorreporting_api
pub struct Clouderrorreporting_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Clouderrorreporting_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group_stat resource handler
    pub fn group_stat(&self) -> resources::Group_stat<'_> {
        resources::Group_stat::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
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
