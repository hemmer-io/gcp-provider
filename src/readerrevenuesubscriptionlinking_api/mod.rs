//! Readerrevenuesubscriptionlinking_api Service
//!
//! Auto-generated service module for readerrevenuesubscriptionlinking_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for readerrevenuesubscriptionlinking_api
pub struct Readerrevenuesubscriptionlinking_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Readerrevenuesubscriptionlinking_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get reader resource handler
    pub fn reader(&self) -> resources::Reader<'_> {
        resources::Reader::new(self.provider)
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
