//! Versionhistory Service
//!
//! Auto-generated service module for versionhistory

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for versionhistory
pub struct VersionhistoryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VersionhistoryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get platform resource handler
    pub fn platform(&self) -> resources::Platform<'_> {
        resources::Platform::new(self.provider)
    }
    /// Get release resource handler
    pub fn release(&self) -> resources::Release<'_> {
        resources::Release::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
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
