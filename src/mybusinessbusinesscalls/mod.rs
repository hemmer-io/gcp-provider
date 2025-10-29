//! Mybusinessbusinesscalls Service
//!
//! Auto-generated service module for mybusinessbusinesscalls

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessbusinesscalls
pub struct MybusinessbusinesscallsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> MybusinessbusinesscallsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get businesscallsinsight resource handler
    pub fn businesscallsinsight(&self) -> resources::Businesscallsinsight<'_> {
        resources::Businesscallsinsight::new(self.provider)
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
