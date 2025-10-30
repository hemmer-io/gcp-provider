//! Playmoviespartner_api Service
//!
//! Auto-generated service module for playmoviespartner_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playmoviespartner_api
pub struct Playmoviespartner_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Playmoviespartner_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get order resource handler
    pub fn order(&self) -> resources::Order<'_> {
        resources::Order::new(self.provider)
    }
    /// Get store_info resource handler
    pub fn store_info(&self) -> resources::Store_info<'_> {
        resources::Store_info::new(self.provider)
    }
    /// Get country resource handler
    pub fn country(&self) -> resources::Country<'_> {
        resources::Country::new(self.provider)
    }
    /// Get avail resource handler
    pub fn avail(&self) -> resources::Avail<'_> {
        resources::Avail::new(self.provider)
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
