//! Acceleratedmobilepageurl Service
//!
//! Auto-generated service module for acceleratedmobilepageurl

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for acceleratedmobilepageurl
pub struct AcceleratedmobilepageurlService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AcceleratedmobilepageurlService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get amp_url resource handler
    pub fn amp_url(&self) -> resources::Amp_url<'_> {
        resources::Amp_url::new(self.provider)
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
