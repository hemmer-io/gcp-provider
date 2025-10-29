//! Cloudfunctions Service
//!
//! Auto-generated service module for cloudfunctions

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudfunctions
pub struct CloudfunctionsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudfunctionsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get runtime resource handler
    pub fn runtime(&self) -> resources::Runtime<'_> {
        resources::Runtime::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get function resource handler
    pub fn function(&self) -> resources::Function<'_> {
        resources::Function::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
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
