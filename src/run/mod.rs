//! Run Service
//!
//! Auto-generated service module for run

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for run
pub struct RunService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> RunService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
