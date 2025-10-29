//! Poly Service
//!
//! Auto-generated service module for poly

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for poly
pub struct PolyService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PolyService<'a> {
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
