//! Mybusinessbusinessinformation_api Service
//!
//! Auto-generated service module for mybusinessbusinessinformation_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessbusinessinformation_api
pub struct Mybusinessbusinessinformation_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinessbusinessinformation_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get categorie resource handler
    pub fn categorie(&self) -> resources::Categorie<'_> {
        resources::Categorie::new(self.provider)
    }
    /// Get attribute resource handler
    pub fn attribute(&self) -> resources::Attribute<'_> {
        resources::Attribute::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get chain resource handler
    pub fn chain(&self) -> resources::Chain<'_> {
        resources::Chain::new(self.provider)
    }
    /// Get google_location resource handler
    pub fn google_location(&self) -> resources::Google_location<'_> {
        resources::Google_location::new(self.provider)
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
