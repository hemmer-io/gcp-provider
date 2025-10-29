//! Pagespeedonline Service
//!
//! Auto-generated service module for pagespeedonline

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pagespeedonline
pub struct PagespeedonlineService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PagespeedonlineService<'a> {
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
