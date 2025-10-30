//! Mybusinessbusinesscalls_api Service
//!
//! Auto-generated service module for mybusinessbusinesscalls_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mybusinessbusinesscalls_api
pub struct Mybusinessbusinesscalls_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Mybusinessbusinesscalls_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get businesscallsinsight resource handler
    pub fn businesscallsinsight(&self) -> resources::Businesscallsinsight<'_> {
        resources::Businesscallsinsight::new(self.provider)
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
