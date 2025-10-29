//! Artifactregistry Service
//!
//! Auto-generated service module for artifactregistry

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for artifactregistry
pub struct ArtifactregistryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ArtifactregistryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get tag resource handler
    pub fn tag(&self) -> resources::Tag<'_> {
        resources::Tag::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get file resource handler
    pub fn file(&self) -> resources::File<'_> {
        resources::File::new(self.provider)
    }
    /// Get repositorie resource handler
    pub fn repositorie(&self) -> resources::Repositorie<'_> {
        resources::Repositorie::new(self.provider)
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
