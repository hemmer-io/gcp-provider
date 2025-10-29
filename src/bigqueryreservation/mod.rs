//! Bigqueryreservation Service
//!
//! Auto-generated service module for bigqueryreservation

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigqueryreservation
pub struct BigqueryreservationService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BigqueryreservationService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get reservation resource handler
    pub fn reservation(&self) -> resources::Reservation<'_> {
        resources::Reservation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get assignment resource handler
    pub fn assignment(&self) -> resources::Assignment<'_> {
        resources::Assignment::new(self.provider)
    }
    /// Get capacity_commitment resource handler
    pub fn capacity_commitment(&self) -> resources::Capacity_commitment<'_> {
        resources::Capacity_commitment::new(self.provider)
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
