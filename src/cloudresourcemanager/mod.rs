//! Cloudresourcemanager Service
//!
//! Auto-generated service module for cloudresourcemanager

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudresourcemanager
pub struct CloudresourcemanagerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudresourcemanagerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get effective_tag resource handler
    pub fn effective_tag(&self) -> resources::Effective_tag<'_> {
        resources::Effective_tag::new(self.provider)
    }
    /// Get effective_tag_binding_collection resource handler
    pub fn effective_tag_binding_collection(&self) -> resources::Effective_tag_binding_collection<'_> {
        resources::Effective_tag_binding_collection::new(self.provider)
    }
    /// Get tag_binding_collection resource handler
    pub fn tag_binding_collection(&self) -> resources::Tag_binding_collection<'_> {
        resources::Tag_binding_collection::new(self.provider)
    }
    /// Get tag_binding resource handler
    pub fn tag_binding(&self) -> resources::Tag_binding<'_> {
        resources::Tag_binding::new(self.provider)
    }
    /// Get lien resource handler
    pub fn lien(&self) -> resources::Lien<'_> {
        resources::Lien::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get tag_hold resource handler
    pub fn tag_hold(&self) -> resources::Tag_hold<'_> {
        resources::Tag_hold::new(self.provider)
    }
    /// Get tag_key resource handler
    pub fn tag_key(&self) -> resources::Tag_key<'_> {
        resources::Tag_key::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get capabilitie resource handler
    pub fn capabilitie(&self) -> resources::Capabilitie<'_> {
        resources::Capabilitie::new(self.provider)
    }
    /// Get tag_value resource handler
    pub fn tag_value(&self) -> resources::Tag_value<'_> {
        resources::Tag_value::new(self.provider)
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
