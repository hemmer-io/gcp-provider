//! Replicapoolupdater_api Service
//!
//! Auto-generated service module for replicapoolupdater_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for replicapoolupdater_api
pub struct Replicapoolupdater_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Replicapoolupdater_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get rolling_update resource handler
    pub fn rolling_update(&self) -> resources::Rolling_update<'_> {
        resources::Rolling_update::new(self.provider)
    }
    /// Get zone_operation resource handler
    pub fn zone_operation(&self) -> resources::Zone_operation<'_> {
        resources::Zone_operation::new(self.provider)
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
