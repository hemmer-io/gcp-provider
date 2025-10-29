//! Realtimebidding Service
//!
//! Auto-generated service module for realtimebidding

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for realtimebidding
pub struct RealtimebiddingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> RealtimebiddingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get bidding_function resource handler
    pub fn bidding_function(&self) -> resources::Bidding_function<'_> {
        resources::Bidding_function::new(self.provider)
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
