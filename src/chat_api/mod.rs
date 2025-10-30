//! Chat_api Service
//!
//! Auto-generated service module for chat_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chat_api
pub struct Chat_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Chat_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get space_event resource handler
    pub fn space_event(&self) -> resources::Space_event<'_> {
        resources::Space_event::new(self.provider)
    }
    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get custom_emoji resource handler
    pub fn custom_emoji(&self) -> resources::Custom_emoji<'_> {
        resources::Custom_emoji::new(self.provider)
    }
    /// Get reaction resource handler
    pub fn reaction(&self) -> resources::Reaction<'_> {
        resources::Reaction::new(self.provider)
    }
    /// Get space_notification_setting resource handler
    pub fn space_notification_setting(&self) -> resources::Space_notification_setting<'_> {
        resources::Space_notification_setting::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get thread resource handler
    pub fn thread(&self) -> resources::Thread<'_> {
        resources::Thread::new(self.provider)
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
