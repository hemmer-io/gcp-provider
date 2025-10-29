//! Mirror Service
//!
//! Auto-generated service module for mirror

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mirror
pub struct MirrorService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MirrorService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get contact resource handler
    pub fn contact(&self) -> resources::Contact<'_> {
        resources::Contact::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get timeline resource handler
    pub fn timeline(&self) -> resources::Timeline<'_> {
        resources::Timeline::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get setting resource handler
    pub fn setting(&self) -> resources::Setting<'_> {
        resources::Setting::new(self.provider)
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
