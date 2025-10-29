//! Adexperiencereport Service
//!
//! Auto-generated service module for adexperiencereport

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adexperiencereport
pub struct AdexperiencereportService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AdexperiencereportService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get violating_site resource handler
    pub fn violating_site(&self) -> resources::Violating_site<'_> {
        resources::Violating_site::new(self.provider)
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
