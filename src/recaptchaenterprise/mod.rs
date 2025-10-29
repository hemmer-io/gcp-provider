//! Recaptchaenterprise Service
//!
//! Auto-generated service module for recaptchaenterprise

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for recaptchaenterprise
pub struct RecaptchaenterpriseService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> RecaptchaenterpriseService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get relatedaccountgroupmembership resource handler
    pub fn relatedaccountgroupmembership(&self) -> resources::Relatedaccountgroupmembership<'_> {
        resources::Relatedaccountgroupmembership::new(self.provider)
    }
    /// Get key resource handler
    pub fn key(&self) -> resources::Key<'_> {
        resources::Key::new(self.provider)
    }
    /// Get firewallpolicie resource handler
    pub fn firewallpolicie(&self) -> resources::Firewallpolicie<'_> {
        resources::Firewallpolicie::new(self.provider)
    }
    /// Get assessment resource handler
    pub fn assessment(&self) -> resources::Assessment<'_> {
        resources::Assessment::new(self.provider)
    }
    /// Get relatedaccountgroup resource handler
    pub fn relatedaccountgroup(&self) -> resources::Relatedaccountgroup<'_> {
        resources::Relatedaccountgroup::new(self.provider)
    }
    /// Get membership resource handler
    pub fn membership(&self) -> resources::Membership<'_> {
        resources::Membership::new(self.provider)
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
