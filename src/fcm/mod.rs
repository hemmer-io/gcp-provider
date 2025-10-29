//! Fcm Service
//!
//! Auto-generated service module for fcm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fcm
pub struct FcmService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FcmService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
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
