//! Searchads360 Service
//!
//! Auto-generated service module for searchads360

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for searchads360
pub struct Searchads360Service<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Searchads360Service<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get search_ads360_field resource handler
    pub fn search_ads360_field(&self) -> resources::Search_ads360_field<'_> {
        resources::Search_ads360_field::new(self.provider)
    }
    /// Get search_ads360 resource handler
    pub fn search_ads360(&self) -> resources::Search_ads360<'_> {
        resources::Search_ads360::new(self.provider)
    }
    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get custom_column resource handler
    pub fn custom_column(&self) -> resources::Custom_column<'_> {
        resources::Custom_column::new(self.provider)
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
