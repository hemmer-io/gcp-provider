//! Storage_api Service
//!
//! Auto-generated service module for storage_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for storage_api
pub struct Storage_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Storage_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
    }
    /// Get object_access_control resource handler
    pub fn object_access_control(&self) -> resources::Object_access_control<'_> {
        resources::Object_access_control::new(self.provider)
    }
    /// Get bucket_access_control resource handler
    pub fn bucket_access_control(&self) -> resources::Bucket_access_control<'_> {
        resources::Bucket_access_control::new(self.provider)
    }
    /// Get object resource handler
    pub fn object(&self) -> resources::Object<'_> {
        resources::Object::new(self.provider)
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
