//! Fcmdata Service
//!
//! Auto-generated service module for fcmdata

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fcmdata
pub struct FcmdataService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FcmdataService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get delivery_data resource handler
    pub fn delivery_data(&self) -> resources::Delivery_data<'_> {
        resources::Delivery_data::new(self.provider)
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
