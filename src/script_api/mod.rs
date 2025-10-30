//! Script_api Service
//!
//! Auto-generated service module for script_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for script_api
pub struct Script_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Script_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get processe resource handler
    pub fn processe(&self) -> resources::Processe<'_> {
        resources::Processe::new(self.provider)
    }
    /// Get script resource handler
    pub fn script(&self) -> resources::Script<'_> {
        resources::Script::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
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
