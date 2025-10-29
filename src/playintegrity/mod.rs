//! Playintegrity Service
//!
//! Auto-generated service module for playintegrity

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playintegrity
pub struct PlayintegrityService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PlayintegrityService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get device_recall resource handler
    pub fn device_recall(&self) -> resources::Device_recall<'_> {
        resources::Device_recall::new(self.provider)
    }
    /// Get playintegrity resource handler
    pub fn playintegrity(&self) -> resources::Playintegrity<'_> {
        resources::Playintegrity::new(self.provider)
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
