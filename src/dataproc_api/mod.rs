//! Dataproc_api Service
//!
//! Auto-generated service module for dataproc_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataproc_api
pub struct Dataproc_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Dataproc_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get workflow_template resource handler
    pub fn workflow_template(&self) -> resources::Workflow_template<'_> {
        resources::Workflow_template::new(self.provider)
    }
    /// Get autoscaling_policie resource handler
    pub fn autoscaling_policie(&self) -> resources::Autoscaling_policie<'_> {
        resources::Autoscaling_policie::new(self.provider)
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
