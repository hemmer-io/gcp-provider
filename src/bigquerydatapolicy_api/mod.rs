//! Bigquerydatapolicy_api Service
//!
//! Auto-generated service module for bigquerydatapolicy_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigquerydatapolicy_api
pub struct Bigquerydatapolicy_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bigquerydatapolicy_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get data_policie resource handler
    pub fn data_policie(&self) -> resources::Data_policie<'_> {
        resources::Data_policie::new(self.provider)
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
