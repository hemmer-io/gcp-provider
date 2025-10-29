//! Firebasedatabase Service
//!
//! Auto-generated service module for firebasedatabase

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebasedatabase
pub struct FirebasedatabaseService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebasedatabaseService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
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
