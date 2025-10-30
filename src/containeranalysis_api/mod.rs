//! Containeranalysis_api Service
//!
//! Auto-generated service module for containeranalysis_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for containeranalysis_api
pub struct Containeranalysis_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Containeranalysis_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
