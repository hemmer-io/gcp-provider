//! Apphub_api Service
//!
//! Auto-generated service module for apphub_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apphub_api
pub struct Apphub_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apphub_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get discovered_service resource handler
    pub fn discovered_service(&self) -> resources::Discovered_service<'_> {
        resources::Discovered_service::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get discovered_workload resource handler
    pub fn discovered_workload(&self) -> resources::Discovered_workload<'_> {
        resources::Discovered_workload::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get service_project_attachment resource handler
    pub fn service_project_attachment(&self) -> resources::Service_project_attachment<'_> {
        resources::Service_project_attachment::new(self.provider)
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
