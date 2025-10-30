//! Clouddeploy_api Service
//!
//! Auto-generated service module for clouddeploy_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for clouddeploy_api
pub struct Clouddeploy_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Clouddeploy_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get deploy_policie resource handler
    pub fn deploy_policie(&self) -> resources::Deploy_policie<'_> {
        resources::Deploy_policie::new(self.provider)
    }
    /// Get job_run resource handler
    pub fn job_run(&self) -> resources::Job_run<'_> {
        resources::Job_run::new(self.provider)
    }
    /// Get custom_target_type resource handler
    pub fn custom_target_type(&self) -> resources::Custom_target_type<'_> {
        resources::Custom_target_type::new(self.provider)
    }
    /// Get delivery_pipeline resource handler
    pub fn delivery_pipeline(&self) -> resources::Delivery_pipeline<'_> {
        resources::Delivery_pipeline::new(self.provider)
    }
    /// Get target resource handler
    pub fn target(&self) -> resources::Target<'_> {
        resources::Target::new(self.provider)
    }
    /// Get release resource handler
    pub fn release(&self) -> resources::Release<'_> {
        resources::Release::new(self.provider)
    }
    /// Get rollout resource handler
    pub fn rollout(&self) -> resources::Rollout<'_> {
        resources::Rollout::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get automation_run resource handler
    pub fn automation_run(&self) -> resources::Automation_run<'_> {
        resources::Automation_run::new(self.provider)
    }
    /// Get automation resource handler
    pub fn automation(&self) -> resources::Automation<'_> {
        resources::Automation::new(self.provider)
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
