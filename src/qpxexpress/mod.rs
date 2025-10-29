//! Qpxexpress Service
//!
//! Auto-generated service module for qpxexpress

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for qpxexpress
pub struct QpxexpressService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> QpxexpressService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get trip resource handler
    pub fn trip(&self) -> resources::Trip<'_> {
        resources::Trip::new(self.provider)
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
