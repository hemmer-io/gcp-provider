//! Cloudkms Service
//!
//! Auto-generated service module for cloudkms

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudkms
pub struct CloudkmsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudkmsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get ekm_config resource handler
    pub fn ekm_config(&self) -> resources::Ekm_config<'_> {
        resources::Ekm_config::new(self.provider)
    }
    /// Get crypto_key resource handler
    pub fn crypto_key(&self) -> resources::Crypto_key<'_> {
        resources::Crypto_key::new(self.provider)
    }
    /// Get crypto_key_version resource handler
    pub fn crypto_key_version(&self) -> resources::Crypto_key_version<'_> {
        resources::Crypto_key_version::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get ekm_connection resource handler
    pub fn ekm_connection(&self) -> resources::Ekm_connection<'_> {
        resources::Ekm_connection::new(self.provider)
    }
    /// Get key_ring resource handler
    pub fn key_ring(&self) -> resources::Key_ring<'_> {
        resources::Key_ring::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get key_handle resource handler
    pub fn key_handle(&self) -> resources::Key_handle<'_> {
        resources::Key_handle::new(self.provider)
    }
    /// Get import_job resource handler
    pub fn import_job(&self) -> resources::Import_job<'_> {
        resources::Import_job::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
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
