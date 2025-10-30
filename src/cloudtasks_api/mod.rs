//! Cloudtasks_api Service
//!
//! Auto-generated service module for cloudtasks_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtasks_api
pub struct Cloudtasks_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudtasks_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get queue resource handler
    pub fn queue(&self) -> resources::Queue<'_> {
        resources::Queue::new(self.provider)
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
