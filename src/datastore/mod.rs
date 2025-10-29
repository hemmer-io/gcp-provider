//! Datastore Service
//!
//! Auto-generated service module for datastore

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datastore
pub struct DatastoreService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DatastoreService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
