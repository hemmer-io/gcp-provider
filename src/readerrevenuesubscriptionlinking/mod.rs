//! Readerrevenuesubscriptionlinking Service
//!
//! Auto-generated service module for readerrevenuesubscriptionlinking

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for readerrevenuesubscriptionlinking
pub struct ReaderrevenuesubscriptionlinkingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ReaderrevenuesubscriptionlinkingService<'a> {
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
