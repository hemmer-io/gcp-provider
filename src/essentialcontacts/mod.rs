//! Essentialcontacts Service
//!
//! Auto-generated service module for essentialcontacts

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for essentialcontacts
pub struct EssentialcontactsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> EssentialcontactsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get contact resource handler
    pub fn contact(&self) -> resources::Contact<'_> {
        resources::Contact::new(self.provider)
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
