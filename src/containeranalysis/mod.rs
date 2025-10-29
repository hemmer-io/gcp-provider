//! Containeranalysis Service
//!
//! Auto-generated service module for containeranalysis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for containeranalysis
pub struct ContaineranalysisService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ContaineranalysisService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get occurrence resource handler
    pub fn occurrence(&self) -> resources::Occurrence<'_> {
        resources::Occurrence::new(self.provider)
    }
    /// Get note resource handler
    pub fn note(&self) -> resources::Note<'_> {
        resources::Note::new(self.provider)
    }
    /// Get scan_config resource handler
    pub fn scan_config(&self) -> resources::Scan_config<'_> {
        resources::Scan_config::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
