//! Gmail_api Service
//!
//! Auto-generated service module for gmail_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gmail_api
pub struct Gmail_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Gmail_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get identitie resource handler
    pub fn identitie(&self) -> resources::Identitie<'_> {
        resources::Identitie::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get history resource handler
    pub fn history(&self) -> resources::History<'_> {
        resources::History::new(self.provider)
    }
    /// Get thread resource handler
    pub fn thread(&self) -> resources::Thread<'_> {
        resources::Thread::new(self.provider)
    }
    /// Get draft resource handler
    pub fn draft(&self) -> resources::Draft<'_> {
        resources::Draft::new(self.provider)
    }
    /// Get filter resource handler
    pub fn filter(&self) -> resources::Filter<'_> {
        resources::Filter::new(self.provider)
    }
    /// Get label resource handler
    pub fn label(&self) -> resources::Label<'_> {
        resources::Label::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get delegate resource handler
    pub fn delegate(&self) -> resources::Delegate<'_> {
        resources::Delegate::new(self.provider)
    }
    /// Get forwarding_addresse resource handler
    pub fn forwarding_addresse(&self) -> resources::Forwarding_addresse<'_> {
        resources::Forwarding_addresse::new(self.provider)
    }
    /// Get smime_info resource handler
    pub fn smime_info(&self) -> resources::Smime_info<'_> {
        resources::Smime_info::new(self.provider)
    }
    /// Get send_a resource handler
    pub fn send_a(&self) -> resources::Send_a<'_> {
        resources::Send_a::new(self.provider)
    }
    /// Get keypair resource handler
    pub fn keypair(&self) -> resources::Keypair<'_> {
        resources::Keypair::new(self.provider)
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
