//! Replicapool_api Service
//!
//! Auto-generated service module for replicapool_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for replicapool_api
pub struct Replicapool_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Replicapool_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get replica resource handler
    pub fn replica(&self) -> resources::Replica<'_> {
        resources::Replica::new(self.provider)
    }
    /// Get pool resource handler
    pub fn pool(&self) -> resources::Pool<'_> {
        resources::Pool::new(self.provider)
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
