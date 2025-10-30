//! Contentwarehouse_api Service
//!
//! Auto-generated service module for contentwarehouse_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for contentwarehouse_api
pub struct Contentwarehouse_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Contentwarehouse_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get synonym_set resource handler
    pub fn synonym_set(&self) -> resources::Synonym_set<'_> {
        resources::Synonym_set::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
    }
    /// Get document_link resource handler
    pub fn document_link(&self) -> resources::Document_link<'_> {
        resources::Document_link::new(self.provider)
    }
    /// Get rule_set resource handler
    pub fn rule_set(&self) -> resources::Rule_set<'_> {
        resources::Rule_set::new(self.provider)
    }
    /// Get document_schema resource handler
    pub fn document_schema(&self) -> resources::Document_schema<'_> {
        resources::Document_schema::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get reference_id resource handler
    pub fn reference_id(&self) -> resources::Reference_id<'_> {
        resources::Reference_id::new(self.provider)
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
