//! Calendar_api Service
//!
//! Auto-generated service module for calendar_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for calendar_api
pub struct Calendar_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Calendar_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get acl resource handler
    pub fn acl(&self) -> resources::Acl<'_> {
        resources::Acl::new(self.provider)
    }
    /// Get calendar resource handler
    pub fn calendar(&self) -> resources::Calendar<'_> {
        resources::Calendar::new(self.provider)
    }
    /// Get freebusy resource handler
    pub fn freebusy(&self) -> resources::Freebusy<'_> {
        resources::Freebusy::new(self.provider)
    }
    /// Get color resource handler
    pub fn color(&self) -> resources::Color<'_> {
        resources::Color::new(self.provider)
    }
    /// Get setting resource handler
    pub fn setting(&self) -> resources::Setting<'_> {
        resources::Setting::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get calendar_list resource handler
    pub fn calendar_list(&self) -> resources::Calendar_list<'_> {
        resources::Calendar_list::new(self.provider)
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
