//! Drive_api Service
//!
//! Auto-generated service module for drive_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for drive_api
pub struct Drive_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Drive_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get replie resource handler
    pub fn replie(&self) -> resources::Replie<'_> {
        resources::Replie::new(self.provider)
    }
    /// Get accessproposal resource handler
    pub fn accessproposal(&self) -> resources::Accessproposal<'_> {
        resources::Accessproposal::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get file resource handler
    pub fn file(&self) -> resources::File<'_> {
        resources::File::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get drive resource handler
    pub fn drive(&self) -> resources::Drive<'_> {
        resources::Drive::new(self.provider)
    }
    /// Get about resource handler
    pub fn about(&self) -> resources::About<'_> {
        resources::About::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get change resource handler
    pub fn change(&self) -> resources::Change<'_> {
        resources::Change::new(self.provider)
    }
    /// Get teamdrive resource handler
    pub fn teamdrive(&self) -> resources::Teamdrive<'_> {
        resources::Teamdrive::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
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
