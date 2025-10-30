//! Bigqueryreservation_api Service
//!
//! Auto-generated service module for bigqueryreservation_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigqueryreservation_api
pub struct Bigqueryreservation_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bigqueryreservation_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get assignment resource handler
    pub fn assignment(&self) -> resources::Assignment<'_> {
        resources::Assignment::new(self.provider)
    }
    /// Get capacity_commitment resource handler
    pub fn capacity_commitment(&self) -> resources::Capacity_commitment<'_> {
        resources::Capacity_commitment::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get reservation resource handler
    pub fn reservation(&self) -> resources::Reservation<'_> {
        resources::Reservation::new(self.provider)
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
