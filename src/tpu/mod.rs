//! Tpu Service
//!
//! Auto-generated service module for tpu

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for tpu
pub struct TpuService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TpuService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get runtime_version resource handler
    pub fn runtime_version(&self) -> resources::Runtime_version<'_> {
        resources::Runtime_version::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get queued_resource resource handler
    pub fn queued_resource(&self) -> resources::Queued_resource<'_> {
        resources::Queued_resource::new(self.provider)
    }
    /// Get accelerator_type resource handler
    pub fn accelerator_type(&self) -> resources::Accelerator_type<'_> {
        resources::Accelerator_type::new(self.provider)
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
