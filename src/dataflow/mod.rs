//! Dataflow Service
//!
//! Auto-generated service module for dataflow

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataflow
pub struct DataflowService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DataflowService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get stage resource handler
    pub fn stage(&self) -> resources::Stage<'_> {
        resources::Stage::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get debug resource handler
    pub fn debug(&self) -> resources::Debug<'_> {
        resources::Debug::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get flex_template resource handler
    pub fn flex_template(&self) -> resources::Flex_template<'_> {
        resources::Flex_template::new(self.provider)
    }
    /// Get work_item resource handler
    pub fn work_item(&self) -> resources::Work_item<'_> {
        resources::Work_item::new(self.provider)
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
