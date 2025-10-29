//! Pubsublite Service
//!
//! Auto-generated service module for pubsublite

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pubsublite
pub struct PubsubliteService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PubsubliteService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
    }
    /// Get reservation resource handler
    pub fn reservation(&self) -> resources::Reservation<'_> {
        resources::Reservation::new(self.provider)
    }
    /// Get cursor resource handler
    pub fn cursor(&self) -> resources::Cursor<'_> {
        resources::Cursor::new(self.provider)
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
