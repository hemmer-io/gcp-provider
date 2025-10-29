//! Abusiveexperiencereport Service
//!
//! Auto-generated service module for abusiveexperiencereport

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for abusiveexperiencereport
pub struct AbusiveexperiencereportService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AbusiveexperiencereportService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get violating_site resource handler
    pub fn violating_site(&self) -> resources::Violating_site<'_> {
        resources::Violating_site::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
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
