//! Firebasedynamiclinks_api Service
//!
//! Auto-generated service module for firebasedynamiclinks_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebasedynamiclinks_api
pub struct Firebasedynamiclinks_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebasedynamiclinks_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get managed_short_link resource handler
    pub fn managed_short_link(&self) -> resources::Managed_short_link<'_> {
        resources::Managed_short_link::new(self.provider)
    }
    /// Get firebasedynamiclink resource handler
    pub fn firebasedynamiclink(&self) -> resources::Firebasedynamiclink<'_> {
        resources::Firebasedynamiclink::new(self.provider)
    }
    /// Get short_link resource handler
    pub fn short_link(&self) -> resources::Short_link<'_> {
        resources::Short_link::new(self.provider)
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
