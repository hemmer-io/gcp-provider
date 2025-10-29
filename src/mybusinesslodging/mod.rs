//! Mybusinesslodging Service
//!
//! Auto-generated service module for mybusinesslodging

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinesslodging
pub struct MybusinesslodgingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MybusinesslodgingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get lodging resource handler
    pub fn lodging(&self) -> resources::Lodging<'_> {
        resources::Lodging::new(self.provider)
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
