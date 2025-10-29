//! Pubsub Service
//!
//! Auto-generated service module for pubsub

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pubsub
pub struct PubsubService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PubsubService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
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
