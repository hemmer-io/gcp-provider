//! Bigquerydatatransfer Service
//!
//! Auto-generated service module for bigquerydatatransfer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigquerydatatransfer
pub struct BigquerydatatransferService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BigquerydatatransferService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get transfer_log resource handler
    pub fn transfer_log(&self) -> resources::Transfer_log<'_> {
        resources::Transfer_log::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get transfer_config resource handler
    pub fn transfer_config(&self) -> resources::Transfer_config<'_> {
        resources::Transfer_config::new(self.provider)
    }
    /// Get run resource handler
    pub fn run(&self) -> resources::Run<'_> {
        resources::Run::new(self.provider)
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
