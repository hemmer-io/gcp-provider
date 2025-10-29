//! Dataproc Service
//!
//! Auto-generated service module for dataproc

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataproc
pub struct DataprocService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DataprocService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get autoscaling_policie resource handler
    pub fn autoscaling_policie(&self) -> resources::Autoscaling_policie<'_> {
        resources::Autoscaling_policie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get workflow_template resource handler
    pub fn workflow_template(&self) -> resources::Workflow_template<'_> {
        resources::Workflow_template::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
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
