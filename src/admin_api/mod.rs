//! Admin_api Service
//!
//! Auto-generated service module for admin_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for admin_api
pub struct Admin_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Admin_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get customer_usage_report resource handler
    pub fn customer_usage_report(&self) -> resources::Customer_usage_report<'_> {
        resources::Customer_usage_report::new(self.provider)
    }
    /// Get user_usage_report resource handler
    pub fn user_usage_report(&self) -> resources::User_usage_report<'_> {
        resources::User_usage_report::new(self.provider)
    }
    /// Get activitie resource handler
    pub fn activitie(&self) -> resources::Activitie<'_> {
        resources::Activitie::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get entity_usage_report resource handler
    pub fn entity_usage_report(&self) -> resources::Entity_usage_report<'_> {
        resources::Entity_usage_report::new(self.provider)
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
