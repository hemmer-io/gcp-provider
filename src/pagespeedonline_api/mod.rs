//! Pagespeedonline_api Service
//!
//! Auto-generated service module for pagespeedonline_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pagespeedonline_api
pub struct Pagespeedonline_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Pagespeedonline_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get pagespeedapi resource handler
    pub fn pagespeedapi(&self) -> resources::Pagespeedapi<'_> {
        resources::Pagespeedapi::new(self.provider)
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
