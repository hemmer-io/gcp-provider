//! Searchads360_api Service
//!
//! Auto-generated service module for searchads360_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for searchads360_api
pub struct Searchads360_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Searchads360_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get custom_column resource handler
    pub fn custom_column(&self) -> resources::Custom_column<'_> {
        resources::Custom_column::new(self.provider)
    }
    /// Get search_ads360 resource handler
    pub fn search_ads360(&self) -> resources::Search_ads360<'_> {
        resources::Search_ads360::new(self.provider)
    }
    /// Get search_ads360_field resource handler
    pub fn search_ads360_field(&self) -> resources::Search_ads360_field<'_> {
        resources::Search_ads360_field::new(self.provider)
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
