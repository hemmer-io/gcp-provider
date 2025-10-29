//! Replicapool Service
//!
//! Auto-generated service module for replicapool

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for replicapool
pub struct ReplicapoolService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ReplicapoolService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get pool resource handler
    pub fn pool(&self) -> resources::Pool<'_> {
        resources::Pool::new(self.provider)
    }
    /// Get replica resource handler
    pub fn replica(&self) -> resources::Replica<'_> {
        resources::Replica::new(self.provider)
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
