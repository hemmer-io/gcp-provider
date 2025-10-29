//! Proximitybeacon Service
//!
//! Auto-generated service module for proximitybeacon

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for proximitybeacon
pub struct ProximitybeaconService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ProximitybeaconService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get proximitybeacon resource handler
    pub fn proximitybeacon(&self) -> resources::Proximitybeacon<'_> {
        resources::Proximitybeacon::new(self.provider)
    }
    /// Get beacon resource handler
    pub fn beacon(&self) -> resources::Beacon<'_> {
        resources::Beacon::new(self.provider)
    }
    /// Get diagnostic resource handler
    pub fn diagnostic(&self) -> resources::Diagnostic<'_> {
        resources::Diagnostic::new(self.provider)
    }
    /// Get beaconinfo resource handler
    pub fn beaconinfo(&self) -> resources::Beaconinfo<'_> {
        resources::Beaconinfo::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
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
