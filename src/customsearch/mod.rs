//! Customsearch Service
//!
//! Auto-generated service module for customsearch

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for customsearch
pub struct CustomsearchService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CustomsearchService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get siterestrict resource handler
    pub fn siterestrict(&self) -> resources::Siterestrict<'_> {
        resources::Siterestrict::new(self.provider)
    }
    /// Get cse resource handler
    pub fn cse(&self) -> resources::Cse<'_> {
        resources::Cse::new(self.provider)
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
