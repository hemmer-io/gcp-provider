//! Partners_api Service
//!
//! Auto-generated service module for partners_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for partners_api
pub struct Partners_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Partners_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get companie resource handler
    pub fn companie(&self) -> resources::Companie<'_> {
        resources::Companie::new(self.provider)
    }
    /// Get offer resource handler
    pub fn offer(&self) -> resources::Offer<'_> {
        resources::Offer::new(self.provider)
    }
    /// Get analytic resource handler
    pub fn analytic(&self) -> resources::Analytic<'_> {
        resources::Analytic::new(self.provider)
    }
    /// Get partner resource handler
    pub fn partner(&self) -> resources::Partner<'_> {
        resources::Partner::new(self.provider)
    }
    /// Get client_message resource handler
    pub fn client_message(&self) -> resources::Client_message<'_> {
        resources::Client_message::new(self.provider)
    }
    /// Get lead resource handler
    pub fn lead(&self) -> resources::Lead<'_> {
        resources::Lead::new(self.provider)
    }
    /// Get history resource handler
    pub fn history(&self) -> resources::History<'_> {
        resources::History::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get user_state resource handler
    pub fn user_state(&self) -> resources::User_state<'_> {
        resources::User_state::new(self.provider)
    }
    /// Get user_event resource handler
    pub fn user_event(&self) -> resources::User_event<'_> {
        resources::User_event::new(self.provider)
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
