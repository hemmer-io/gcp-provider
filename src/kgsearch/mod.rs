//! Kgsearch Service
//!
//! Auto-generated service module for kgsearch

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kgsearch
pub struct KgsearchService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> KgsearchService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get entitie resource handler
    pub fn entitie(&self) -> resources::Entitie<'_> {
        resources::Entitie::new(self.provider)
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
