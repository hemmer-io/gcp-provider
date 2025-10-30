//! Websecurityscanner_api Service
//!
//! Auto-generated service module for websecurityscanner_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for websecurityscanner_api
pub struct Websecurityscanner_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Websecurityscanner_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get scan_config resource handler
    pub fn scan_config(&self) -> resources::Scan_config<'_> {
        resources::Scan_config::new(self.provider)
    }
    /// Get finding_type_stat resource handler
    pub fn finding_type_stat(&self) -> resources::Finding_type_stat<'_> {
        resources::Finding_type_stat::new(self.provider)
    }
    /// Get scan_run resource handler
    pub fn scan_run(&self) -> resources::Scan_run<'_> {
        resources::Scan_run::new(self.provider)
    }
    /// Get crawled_url resource handler
    pub fn crawled_url(&self) -> resources::Crawled_url<'_> {
        resources::Crawled_url::new(self.provider)
    }
    /// Get finding resource handler
    pub fn finding(&self) -> resources::Finding<'_> {
        resources::Finding::new(self.provider)
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
