//! Tagmanager Service
//!
//! Auto-generated service module for tagmanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for tagmanager
pub struct TagmanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TagmanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get destination resource handler
    pub fn destination(&self) -> resources::Destination<'_> {
        resources::Destination::new(self.provider)
    }
    /// Get workspace resource handler
    pub fn workspace(&self) -> resources::Workspace<'_> {
        resources::Workspace::new(self.provider)
    }
    /// Get client resource handler
    pub fn client(&self) -> resources::Client<'_> {
        resources::Client::new(self.provider)
    }
    /// Get trigger resource handler
    pub fn trigger(&self) -> resources::Trigger<'_> {
        resources::Trigger::new(self.provider)
    }
    /// Get user_permission resource handler
    pub fn user_permission(&self) -> resources::User_permission<'_> {
        resources::User_permission::new(self.provider)
    }
    /// Get gtag_config resource handler
    pub fn gtag_config(&self) -> resources::Gtag_config<'_> {
        resources::Gtag_config::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
    }
    /// Get built_in_variable resource handler
    pub fn built_in_variable(&self) -> resources::Built_in_variable<'_> {
        resources::Built_in_variable::new(self.provider)
    }
    /// Get variable resource handler
    pub fn variable(&self) -> resources::Variable<'_> {
        resources::Variable::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get version_header resource handler
    pub fn version_header(&self) -> resources::Version_header<'_> {
        resources::Version_header::new(self.provider)
    }
    /// Get container resource handler
    pub fn container(&self) -> resources::Container<'_> {
        resources::Container::new(self.provider)
    }
    /// Get transformation resource handler
    pub fn transformation(&self) -> resources::Transformation<'_> {
        resources::Transformation::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get tag resource handler
    pub fn tag(&self) -> resources::Tag<'_> {
        resources::Tag::new(self.provider)
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
