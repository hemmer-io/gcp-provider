//! Poly_api Service
//!
//! Auto-generated service module for poly_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for poly_api
pub struct Poly_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Poly_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get likedasset resource handler
    pub fn likedasset(&self) -> resources::Likedasset<'_> {
        resources::Likedasset::new(self.provider)
    }
    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
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
