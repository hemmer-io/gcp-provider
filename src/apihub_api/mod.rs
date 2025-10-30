//! Apihub_api Service
//!
//! Auto-generated service module for apihub_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apihub_api
pub struct Apihub_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apihub_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get discovered_api_operation resource handler
    pub fn discovered_api_operation(&self) -> resources::Discovered_api_operation<'_> {
        resources::Discovered_api_operation::new(self.provider)
    }
    /// Get dependencie resource handler
    pub fn dependencie(&self) -> resources::Dependencie<'_> {
        resources::Dependencie::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get runtime_project_attachment resource handler
    pub fn runtime_project_attachment(&self) -> resources::Runtime_project_attachment<'_> {
        resources::Runtime_project_attachment::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get external_api resource handler
    pub fn external_api(&self) -> resources::External_api<'_> {
        resources::External_api::new(self.provider)
    }
    /// Get api_hub_instance resource handler
    pub fn api_hub_instance(&self) -> resources::Api_hub_instance<'_> {
        resources::Api_hub_instance::new(self.provider)
    }
    /// Get plugin resource handler
    pub fn plugin(&self) -> resources::Plugin<'_> {
        resources::Plugin::new(self.provider)
    }
    /// Get style_guide resource handler
    pub fn style_guide(&self) -> resources::Style_guide<'_> {
        resources::Style_guide::new(self.provider)
    }
    /// Get spec resource handler
    pub fn spec(&self) -> resources::Spec<'_> {
        resources::Spec::new(self.provider)
    }
    /// Get attribute resource handler
    pub fn attribute(&self) -> resources::Attribute<'_> {
        resources::Attribute::new(self.provider)
    }
    /// Get discovered_api_observation resource handler
    pub fn discovered_api_observation(&self) -> resources::Discovered_api_observation<'_> {
        resources::Discovered_api_observation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get definition resource handler
    pub fn definition(&self) -> resources::Definition<'_> {
        resources::Definition::new(self.provider)
    }
    /// Get curation resource handler
    pub fn curation(&self) -> resources::Curation<'_> {
        resources::Curation::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get host_project_registration resource handler
    pub fn host_project_registration(&self) -> resources::Host_project_registration<'_> {
        resources::Host_project_registration::new(self.provider)
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
