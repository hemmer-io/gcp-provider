//! Logging_api Service
//!
//! Auto-generated service module for logging_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for logging_api
pub struct Logging_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Logging_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get metric resource handler
    pub fn metric(&self) -> resources::Metric<'_> {
        resources::Metric::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get view resource handler
    pub fn view(&self) -> resources::View<'_> {
        resources::View::new(self.provider)
    }
    /// Get exclusion resource handler
    pub fn exclusion(&self) -> resources::Exclusion<'_> {
        resources::Exclusion::new(self.provider)
    }
    /// Get log_scope resource handler
    pub fn log_scope(&self) -> resources::Log_scope<'_> {
        resources::Log_scope::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
    }
    /// Get entrie resource handler
    pub fn entrie(&self) -> resources::Entrie<'_> {
        resources::Entrie::new(self.provider)
    }
    /// Get log resource handler
    pub fn log(&self) -> resources::Log<'_> {
        resources::Log::new(self.provider)
    }
    /// Get monitored_resource_descriptor resource handler
    pub fn monitored_resource_descriptor(&self) -> resources::Monitored_resource_descriptor<'_> {
        resources::Monitored_resource_descriptor::new(self.provider)
    }
    /// Get recent_querie resource handler
    pub fn recent_querie(&self) -> resources::Recent_querie<'_> {
        resources::Recent_querie::new(self.provider)
    }
    /// Get logging resource handler
    pub fn logging(&self) -> resources::Logging<'_> {
        resources::Logging::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get saved_querie resource handler
    pub fn saved_querie(&self) -> resources::Saved_querie<'_> {
        resources::Saved_querie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get sink resource handler
    pub fn sink(&self) -> resources::Sink<'_> {
        resources::Sink::new(self.provider)
    }
    /// Get link resource handler
    pub fn link(&self) -> resources::Link<'_> {
        resources::Link::new(self.provider)
    }
    /// Get billing_account resource handler
    pub fn billing_account(&self) -> resources::Billing_account<'_> {
        resources::Billing_account::new(self.provider)
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
