//! File_api Service
//!
//! Auto-generated service module for file_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for file_api
pub struct File_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> File_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get share resource handler
    pub fn share(&self) -> resources::Share<'_> {
        resources::Share::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
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
