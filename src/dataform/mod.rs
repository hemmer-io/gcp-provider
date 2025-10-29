//! Dataform Service
//!
//! Auto-generated service module for dataform

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataform
pub struct DataformService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DataformService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get compilation_result resource handler
    pub fn compilation_result(&self) -> resources::Compilation_result<'_> {
        resources::Compilation_result::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get team_folder resource handler
    pub fn team_folder(&self) -> resources::Team_folder<'_> {
        resources::Team_folder::new(self.provider)
    }
    /// Get workflow_config resource handler
    pub fn workflow_config(&self) -> resources::Workflow_config<'_> {
        resources::Workflow_config::new(self.provider)
    }
    /// Get repositorie resource handler
    pub fn repositorie(&self) -> resources::Repositorie<'_> {
        resources::Repositorie::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get workspace resource handler
    pub fn workspace(&self) -> resources::Workspace<'_> {
        resources::Workspace::new(self.provider)
    }
    /// Get release_config resource handler
    pub fn release_config(&self) -> resources::Release_config<'_> {
        resources::Release_config::new(self.provider)
    }
    /// Get workflow_invocation resource handler
    pub fn workflow_invocation(&self) -> resources::Workflow_invocation<'_> {
        resources::Workflow_invocation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
