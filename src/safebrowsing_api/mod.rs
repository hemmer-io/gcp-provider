//! Safebrowsing_api Service
//!
//! Auto-generated service module for safebrowsing_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for safebrowsing_api
pub struct Safebrowsing_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Safebrowsing_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get full_hashe resource handler
    pub fn full_hashe(&self) -> resources::Full_hashe<'_> {
        resources::Full_hashe::new(self.provider)
    }
    /// Get threat_matche resource handler
    pub fn threat_matche(&self) -> resources::Threat_matche<'_> {
        resources::Threat_matche::new(self.provider)
    }
    /// Get encoded_full_hashe resource handler
    pub fn encoded_full_hashe(&self) -> resources::Encoded_full_hashe<'_> {
        resources::Encoded_full_hashe::new(self.provider)
    }
    /// Get threat_list_update resource handler
    pub fn threat_list_update(&self) -> resources::Threat_list_update<'_> {
        resources::Threat_list_update::new(self.provider)
    }
    /// Get threat_list resource handler
    pub fn threat_list(&self) -> resources::Threat_list<'_> {
        resources::Threat_list::new(self.provider)
    }
    /// Get encoded_update resource handler
    pub fn encoded_update(&self) -> resources::Encoded_update<'_> {
        resources::Encoded_update::new(self.provider)
    }
    /// Get threat_hit resource handler
    pub fn threat_hit(&self) -> resources::Threat_hit<'_> {
        resources::Threat_hit::new(self.provider)
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
