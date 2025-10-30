//! People_api Service
//!
//! Auto-generated service module for people_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for people_api
pub struct People_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> People_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get contact_group resource handler
    pub fn contact_group(&self) -> resources::Contact_group<'_> {
        resources::Contact_group::new(self.provider)
    }
    /// Get other_contact resource handler
    pub fn other_contact(&self) -> resources::Other_contact<'_> {
        resources::Other_contact::new(self.provider)
    }
    /// Get people resource handler
    pub fn people(&self) -> resources::People<'_> {
        resources::People::new(self.provider)
    }
    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
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
